use crate::character::request::StateRequest;
use crate::character::snapshot::CharacterSnapshot;
use crate::character::CharacterId;
use crate::map::LogicMap;
use crate::map::StepType;
use crate::CharacterLogic;
use platform::types::{Direction, Vector2D, Vector2Di};
use platform::Animator;
use std::sync::Arc;

pub struct ScriptedCharacterLogic {
    base: CharacterLogic,
}

impl ScriptedCharacterLogic {
    pub fn new(id: CharacterId, animator: Box<dyn Animator>) -> Self {
        ScriptedCharacterLogic {
            base: CharacterLogic::new(id, animator),
        }
    }

    pub fn process(&mut self, delta: f32, logic_map: &Arc<LogicMap>) {
        self.base.process(delta, logic_map);
    }

    pub fn set_logic_map(&mut self, map: Arc<LogicMap>) {
        self.base.set_logic_map(map);
    }

    pub fn set_cell_position(&mut self, i: i32, j: i32) -> Vector2Di {
        self.base.set_cell_position(i, j)
    }

    pub fn get_position(&self) -> Vector2D {
        self.base.get_position()
    }

    pub fn snapshot(&self) -> CharacterSnapshot {
        self.base.snapshot()
    }

    // check if the character is in the idle state
    pub fn is_idle(&self) -> bool {
        self.base.is_idle()
    }

    pub fn get_direction(&self) -> Direction {
        self.base.direction
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.base.direction = direction;
    }

    /*
     * Thread-safe method to request a state change.
     * Can be called from Input, Behaviour Tree, or other threads
     */
    pub fn request_state(&self, request: StateRequest) {
        self.base.request_state(request);
    }

    pub fn try_transition(&mut self, req: StateRequest) -> Result<(), String> {
        self.base.try_transition(req)
    }

    pub fn get_cell_position(&self) -> Vector2Di {
        self.base.get_cell_position()
    }

    // Reset character to its initial state (position, FSM, BT blackboard)
    pub fn reset(&mut self) {
        self.base.reset();
    }

    pub fn set_start_cell(&mut self, cell: Vector2Di) {
        self.base.start_cell = cell;
    }

    pub fn snap_to_cell(&mut self) -> Vector2Di {
        self.base.snap_to_cell()
    }

    /// Recursively resolves the final position after traversing a staircase.
    ///
    /// Checks whether `cell` contains stairs. If it does, and the stairs can be
    /// traversed in the given `direction`, the function advances by the stair
    /// offset and calls itself on the next cell. This repeats until a non-stair
    /// cell is reached or the stairs cannot be traversed in the given direction.
    ///
    /// # Arguments
    ///
    /// * `cell`      - The cell to inspect for stairs.
    /// * `direction` - The direction of travel, used to determine whether the
    ///   staircase can be entered and to select the correct offset.
    ///
    /// # Returns
    ///
    /// A [`Vector2Di`] representing the resolved position:
    /// - The **original** `cell`, if it contains no stairs.
    /// - The **cell at the far end** of the staircase, if the stairs were
    ///   successfully traversed.
    /// - The **current** `cell`, if the stairs exist but cannot be traversed
    ///   in the given direction (e.g. approaching from the wrong side).
    ///
    /// # Notes
    ///
    /// This function is recursive. Each call advances one stair segment which consists of two vertical tiles, so
    /// the recursion depth equals the number of stair segments in the chain.
    pub fn check_stairs(&self, cell: Vector2Di, direction: &Direction) -> Vector2Di {
        let step_type = self.base.logic_map.get_step_type(cell);

        match step_type {
            StepType::None => cell,
            _ => {
                if let Some(offset_vec) =
                    CharacterLogic::get_steps_offset_vector(step_type, direction)
                {
                    let new_coord = cell + offset_vec;
                    self.check_stairs(new_coord, direction)
                } else {
                    cell
                }
            }
        }
    }

    ///Attemps to move the character one step in the given direction.
    ///
    /// Resolves the resulting position accounting the following rules:
    /// - If the target cell is **not walkable**, the character stays in place and the current position is returned
    /// - If the target cell contains **stairs**, the character traverses the entire staircase in a single step, and
    ///   the position of the cell next to the beginning (or the nd of) stairs is returned
    ///
    /// # Arguments
    /// * `direction`- The direction in which the character attempts to move.
    ///
    /// # Returns
    /// A [`Option(Vector2Di)`] representing the resolved cell position after the step:
    /// - The position at the end of (or beginning) the stairs, if stairs were encountered.
    /// - The adjacent cell position, if the move succeeded without stairs.
    /// - **None**, if the move was blocked.
    ///
    /// # Example
    /// ```ignore
    /// let new_pos = character.try_step(&Direction::North);
    /// if new_pos == character.get_cell_position() {
    ///     println!("Move was blocked");
    /// }
    /// ```
    pub fn try_step(&self, direction: &Direction) -> Option<Vector2Di> {
        let current_position = self.get_cell_position();
        let next_cell = current_position + direction.to_vector();

        //check is the next cell is walkable
        if self
            .base
            .logic_map
            .is_walkable_from(self.get_cell_position(), next_cell)
        {
            Some(self.check_stairs(next_cell, direction))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::logic_map::{LogicCell, LogicMap};
    use crate::map::StepType;

    fn ensure_init() {
        crate::test_utils::test_init::ensure_init();
    }

    struct TestAnimator {
        position: Vector2D,
        animation: String,
        playing: bool,
    }

    impl TestAnimator {
        fn new(position: Vector2D) -> Self {
            Self {
                position,
                animation: String::new(),
                playing: false,
            }
        }
    }

    impl Animator for TestAnimator {
        fn play(&mut self, name: &str) {
            self.animation = name.to_string();
            self.playing = true;
        }
        fn is_playing(&self) -> bool {
            self.playing
        }
        fn process(&mut self, _delta: f32) {}
        fn set_position(&mut self, position: Vector2D) {
            self.position = position;
        }
        fn get_position(&self) -> Vector2D {
            self.position
        }
        fn get_global_position(&self) -> Vector2D {
            self.position
        }
    }

    /// 6×2 map for horizontal step traversal:
    /// Col:  0        1        2        3        4        5
    ///   0:  flat,h=0 flat,h=0 step,h=1 flat,h=1 flat,h=1
    ///   1:  flat,h=0 flat,h=0 step,h=0 flat,h=1 flat,h=1
    fn make_test_map() -> Arc<LogicMap> {
        let mut map = LogicMap::new(6, 2);
        for j in 0..2 {
            for i in 0..6 {
                let (height, step_type) = match (i, j) {
                    (0 | 1, 0 | 1) => (0, StepType::None),
                    (3 | 4, 0 | 1) => (1, StepType::None),
                    (2, 0) => (1, StepType::Left),
                    (2, 1) => (0, StepType::Left),
                    _ => (0, StepType::None),
                };
                map.set_cell(
                    i,
                    j,
                    Some(LogicCell {
                        walkable: true,
                        height,
                        step_type,
                    }),
                );
            }
        }
        Arc::new(map)
    }

    fn make_character(cell_x: i32, cell_y: i32, map: &Arc<LogicMap>) -> ScriptedCharacterLogic {
        ensure_init();
        let cell_size: f32 = 64.0;
        let pos = Vector2D {
            x: cell_x as f32 * cell_size + cell_size / 2.0,
            y: cell_y as f32 * cell_size + cell_size / 2.0,
        };
        let mut ch = ScriptedCharacterLogic::new(
            cell_x as u32 * 1000 + cell_y as u32,
            Box::new(TestAnimator::new(pos)),
        );
        ch.set_logic_map(map.clone());
        ch.set_cell_position(cell_x, cell_y);

        // ch.prev_cell = Vector2Di::new(cell_x, cell_y);
        ch
    }

    // --- going on step left downstairs ---

    #[test]
    fn test_go_left_downstairs() {
        let map = make_test_map();
        let ch = make_character(3, 0, &map);

        let target = ch.try_step(&Direction::WEST);
        assert_eq!(target, Some(Vector2Di::new(1, 1)));
    }

    #[test]
    fn test_go_right_upstairs() {
        let map = make_test_map();
        let ch = make_character(1, 1, &map);

        let target = ch.try_step(&Direction::EAST);
        assert_eq!(target, Some(Vector2Di::new(3, 0)));
    }
}
