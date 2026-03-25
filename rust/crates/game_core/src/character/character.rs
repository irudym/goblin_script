use crate::bt::Blackboard;
use crate::character::request::StateRequest;
use crate::character::snapshot::CharacterSnapshot;
use crate::character::CharacterId;
use crate::fsm::FSM;
use crate::fsm::{IdleState, RunState, TurnState, WaitState, WalkState};
use crate::StateType;
use platform::logger::LogType;
use platform::types::Vector2Di;
use platform::types::{Direction, Vector2D};
use platform::Animator;
use std::sync::{Arc, Mutex};

use platform::log_debug;

use crate::map::{LogicMap, StepType};

pub struct CharacterLogic {
    pub direction: Direction,
    pub start_direction: Direction,
    pub speed: f32,
    pub current_speed: f32,
    state: Option<Box<dyn FSM>>, // the active state machine, accessible only by Main Thread

    pending_request: Arc<Mutex<Option<StateRequest>>>, // the request buffer, thread safe

    animator: Box<dyn Animator>,

    // cell_size: f32, //default value: 32px

    // pending_commands: Vec<BTCommand>,
    pub id: CharacterId,
    pub blackboard: Box<Blackboard>,

    prev_cell: Vector2Di,
    pub start_cell: Vector2Di, // initial coordinates, used during level reset.
    pub logic_map: Arc<LogicMap>,
}

impl CharacterLogic {
    pub fn new(id: CharacterId, animator: Box<dyn Animator>) -> Self {
        Self {
            id,
            direction: Direction::SOUTH,
            start_direction: Direction::SOUTH,
            speed: 100.0,
            current_speed: 0.0,
            state: None,
            pending_request: Arc::new(Mutex::new(Some(StateRequest::Idle))),
            animator,
            logic_map: Arc::new(LogicMap::new(0, 0)),
            blackboard: Box::new(Blackboard::new()),

            prev_cell: Vector2Di::new(0, 0),
            start_cell: Vector2Di::new(0, 0),
        }
    }

    pub fn set_logic_map(&mut self, map: Arc<LogicMap>) {
        self.logic_map = map;
    }

    pub fn set_initial_values(&mut self, start_cell: Vector2Di, start_direction: Direction) {
        self.start_cell = start_cell;
        self.start_direction = start_direction;
    }

    // Snap the character to the cell coordinates
    // Return new cell coordinates
    pub fn snap_to_cell(&mut self) -> Vector2Di {
        // get cell i,j
        let coord = self.get_cell_position();
        self.set_cell_position(coord.x, coord.y);
        coord
    }

    pub fn get_position(&self) -> Vector2D {
        self.animator.get_position()
    }

    pub fn set_position(&mut self, position: Vector2D) {
        self.animator.set_position(position);
    }

    // Set character position in tile grid coordinates: I,J, reset the prev_cell as well
    pub fn set_cell_position(&mut self, i: i32, j: i32) -> Vector2Di {
        let position = Vector2Di { x: i, y: j };
        let screen_pos = self.logic_map.get_screen_position(position);

        self.set_position(screen_pos);

        self.prev_cell = position;
        position
    }

    //get character cell position in tile grid coordinates: I,J
    pub fn get_cell_position(&self) -> Vector2Di {
        let position = self.animator.get_global_position();
        self.logic_map.get_cell_position(position)
    }

    // check if the character is in the idle state
    pub fn is_idle(&self) -> bool {
        if let Some(val) = self.state.as_ref() {
            val.get_type() == StateType::IDLE
        } else {
            false
        }
    }

    pub fn set_current_speed(&mut self, speed: f32) {
        self.current_speed = speed;
    }

    /*
     * Set and play animation by construction animation name using Character direction:
     * for example, animation is run, the direction is WEST, than animation name will be run_west
     */
    pub fn play_animation_with_direction(&mut self, animation_name: &str) {
        /*
        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        */
        let animation = format!("{}_{}", animation_name, self.direction);
        self.animator.play(&animation);
    }

    pub fn play_animation(&mut self, animation_name: &str) {
        self.animator.play(animation_name);
    }

    pub fn is_animation_playing(&self) -> bool {
        self.animator.is_playing()
    }

    /*
     * Thread-safe method to request a state change.
     * Can be called from Input, Behaviour Tree, or other threads
     */
    pub fn request_state(&self, request: StateRequest) {
        if let Ok(mut pending) = self.pending_request.lock() {
            // "last win strategy" - if multiple systems request a state in the same frame, the last one overrides
            *pending = Some(request);
        }
    }

    /*
     * Internal method to process the queue
     * This acts like a factory that converts enum to structs
     */
    fn handle_transitions(&mut self) {
        //1. Check if there is a request
        let request_opt = {
            let mut lock = self.pending_request.lock().unwrap();
            lock.take()
        };

        if let Some(req) = request_opt {
            let _ = self.try_transition(req);
        }
    }

    /*
     * Transition logic with validation
     */
    pub fn try_transition(&mut self, req: StateRequest) -> Result<(), String> {
        log_debug!("Character[{}]: try_transition to {:?}", self.id, req);
        let can_exit = if let Some(val) = self.state.as_ref() {
            val.can_exit()
        } else {
            true
        };

        //1. Map request -> Target state type
        let target_type = match req {
            StateRequest::Idle => StateType::IDLE,
            StateRequest::Run => StateType::RUN,
            StateRequest::Turn(_) => StateType::TURN,
            StateRequest::WalkTo(_) => StateType::RUN,
            StateRequest::Wait(_) => StateType::WAIT,
        };

        //2. validate transition rules
        if !can_exit {
            // the state is locked
            return Err("Cannot exit from the current state".to_string());
        }

        let new_state: Box<dyn FSM> = CharacterLogic::get_state_by_request(&req);

        // perform the swap
        if let Some(old_state) = self.state.take() {
            let state_type = &old_state.get_type();
            if !old_state.can_transition_to(target_type) {
                self.state = Some(old_state);
                return Err(format!(
                    "Cannot make transition from {:?} to {:?}",
                    state_type, target_type
                ));
            }
            old_state.exit(self);
        }

        let mut next_state = new_state;
        next_state.enter(self);

        self.state = Some(next_state);

        Ok(())
    }

    fn get_state_by_request(req: &StateRequest) -> Box<dyn FSM> {
        match req {
            StateRequest::Idle => Box::new(IdleState::new()),
            StateRequest::Run => Box::new(RunState::new()),
            StateRequest::Turn(direction) => Box::new(TurnState::new(*direction)),
            StateRequest::WalkTo(target) => Box::new(WalkState::new(*target)),
            StateRequest::Wait(time) => Box::new(WaitState::new(*time)),
        }
    }

    // Set the state without validations
    // Can be used to switch character to Idle state
    pub fn force_transition(&mut self, req: StateRequest) {
        let mut new_state: Box<dyn FSM> = CharacterLogic::get_state_by_request(&req);

        // perform the swap
        if let Some(old_state) = self.state.take() {
            old_state.exit(self);
        }

        new_state.enter(self);
        self.state = Some(new_state);
    }

    pub fn snapshot(&self) -> CharacterSnapshot {
        CharacterSnapshot {
            id: self.id,
            position: self.animator.get_position(),
            direction: self.direction,
            velocity: self.current_speed * self.direction.to_vector(),
            is_idle: self.is_idle(),
            blackboard: self.blackboard.clone(),
            current_speed: self.current_speed,
            cell_position: self.get_cell_position(),
        }
    }

    // return offset vector
    pub fn get_steps_offset_vector(step_type: StepType, direction: &Direction) -> Option<Vector2D> {
        match (step_type, direction) {
            (StepType::Left, Direction::EAST) => Some(Vector2D::new(1.0, -1.0)),
            (StepType::Left, Direction::WEST) => Some(Vector2D::new(-1.0, 1.0)),
            // Right steps: "\" slope — falls eastward
            (StepType::Right, Direction::EAST) => Some(Vector2D::new(1.0, 1.0)),
            (StepType::Right, Direction::WEST) => Some(Vector2D::new(-1.0, -1.0)),
            // No step or NORTH/SOUTH on steps: normal movement
            _ => None,
        }
    }

    fn get_effective_velocity(&self, logic_map: &LogicMap) -> Vector2D {
        let base = self.direction.to_vector() * self.current_speed;

        if self.current_speed == 0.0 {
            return base;
        }

        let cell = logic_map.get_cell_position(self.get_position());

        let step_type = logic_map.get_step_type(cell);

        // On step tiles: east/west movement becomes diagonal
        // Using (speed, -speed) preserves horizontal speed and matches
        // the original offset formula where ΔY = ΔX
        if let Some(offset_vector) =
            CharacterLogic::get_steps_offset_vector(step_type, &self.direction)
        {
            offset_vector * self.current_speed
        } else {
            base
        }
    }

    pub fn process(&mut self, delta: f32, logic_map: &Arc<LogicMap>) {
        let state_type = self.state.as_ref().map(|state| state.get_type());

        log_debug!(
            "Character[{}]::process\nDirection: {}\ncurrent_state: {:?}\ncurrent_pos: {:?}\ncurrent_cell: {:?}",
            self.id,
            self.direction,
            state_type,
            self.get_position(),
            self.get_cell_position(),
        );

        // Process pending request
        self.handle_transitions();

        if self.current_speed > 0.0 {
            let velocity = self.get_effective_velocity(logic_map);
            let new_position = self.get_position() + velocity * delta;
            self.set_position(new_position);
        }

        let mut pos = logic_map.get_cell_position(self.get_position());

        if !logic_map.is_walkable_from(self.prev_cell, pos) {
            // the character got to non walkable cell, set the position to the previous cell
            // and set Idle state
            pos = self.set_cell_position(self.prev_cell.x, self.prev_cell.y);
            // transfer to idle
            self.force_transition(StateRequest::Idle);
        }

        // Update the current state
        if let Some(mut state) = self.state.take() {
            state.update(delta, self);
            self.state = Some(state);
        }

        // Update animation
        self.animator.process(delta);

        self.prev_cell = pos;
    }

    // Reset character to its initial state (position, FSM, BT blackboard)
    pub fn reset(&mut self) {
        // Restore direction
        self.direction = self.start_direction;

        // Restore position
        self.set_cell_position(self.start_cell.x, self.start_cell.y);

        // Force idle state
        self.force_transition(StateRequest::Idle);

        // Clear BT execution state (Blackboard stores sequence/selector indices)
        *self.blackboard = Blackboard::new();

        // Clear any state request
        if let Ok(mut pending) = self.pending_request.lock() {
            *pending = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::logic_map::{LogicCell, LogicMap};

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

    /// 6×1 map for horizontal step traversal:
    /// Col:  0        1        2        3        4        5
    ///       flat,h=0 flat,h=0 step,h=0 step,h=1 flat,h=1 flat,h=1
    fn make_test_map() -> Arc<LogicMap> {
        let mut map = LogicMap::new(6, 1);
        for i in 0..6 {
            let (height, step_type) = match i {
                2 => (0, StepType::Left),
                3 => (1, StepType::Left),
                4 | 5 => (1, StepType::None),
                _ => (0, StepType::None),
            };
            map.set_cell(
                i,
                0,
                Some(LogicCell {
                    walkable: true,
                    height,
                    step_type,
                }),
            );
        }
        Arc::new(map)
    }

    fn make_character(cell_x: i32, cell_y: i32, map: &Arc<LogicMap>) -> CharacterLogic {
        ensure_init();
        let cell_size: f32 = 64.0;
        let pos = Vector2D {
            x: cell_x as f32 * cell_size + cell_size / 2.0,
            y: cell_y as f32 * cell_size + cell_size / 2.0,
        };
        let mut ch = CharacterLogic::new(
            cell_x as u32 * 1000 + cell_y as u32,
            Box::new(TestAnimator::new(pos)),
        );
        ch.set_logic_map(map.clone());
        ch.prev_cell = Vector2Di::new(cell_x, cell_y);
        ch
    }

    // --- effective velocity tests ---

    #[test]
    fn test_effective_velocity_flat_ground_east() {
        let map = make_test_map();
        let mut ch = make_character(0, 0, &map);
        ch.direction = Direction::EAST;
        ch.current_speed = 100.0;
        let v = ch.get_effective_velocity(&map);
        assert_eq!(v, Vector2D::new(100.0, 0.0));
    }

    #[test]
    fn test_effective_velocity_step_east() {
        let map = make_test_map();
        let mut ch = make_character(2, 0, &map);
        ch.direction = Direction::EAST;
        ch.current_speed = 100.0;
        let v = ch.get_effective_velocity(&map);
        assert_eq!(v, Vector2D::new(100.0, -100.0));
    }

    #[test]
    fn test_effective_velocity_step_west() {
        let map = make_test_map();
        let mut ch = make_character(3, 0, &map);
        ch.direction = Direction::WEST;
        ch.current_speed = 100.0;
        let v = ch.get_effective_velocity(&map);
        assert_eq!(v, Vector2D::new(-100.0, 100.0));
    }

    #[test]
    fn test_effective_velocity_step_north() {
        let map = make_test_map();
        let mut ch = make_character(2, 0, &map);
        ch.direction = Direction::NORTH;
        ch.current_speed = 100.0;
        let v = ch.get_effective_velocity(&map);
        // N/S on step tiles → no diagonal, just normal direction
        assert_eq!(v, Vector2D::new(0.0, -100.0));
    }

    // --- process integration tests ---

    #[test]
    fn test_process_step_going_up() {
        let map = make_test_map();
        let mut ch = make_character(2, 0, &map);
        ch.direction = Direction::EAST;
        ch.current_speed = 100.0;
        ch.request_state(StateRequest::Run);

        let start_pos = ch.get_position();
        for _ in 0..3 {
            ch.process(0.016, &map);
        }
        let end_pos = ch.get_position();

        assert!(end_pos.x > start_pos.x, "X should increase going east");
        assert!(end_pos.y < start_pos.y, "Y should decrease going up step");
    }

    #[test]
    fn test_process_step_going_down() {
        let map = make_test_map();
        let mut ch = make_character(3, 0, &map);
        ch.direction = Direction::WEST;
        ch.current_speed = 100.0;
        ch.request_state(StateRequest::Run);

        let start_pos = ch.get_position();
        for _ in 0..3 {
            ch.process(0.016, &map);
        }
        let end_pos = ch.get_position();

        assert!(end_pos.x < start_pos.x, "X should decrease going west");
        assert!(end_pos.y > start_pos.y, "Y should increase going down step");
    }

    #[test]
    fn test_process_flat_ground_no_y_change() {
        let map = make_test_map();
        let mut ch = make_character(0, 0, &map);
        ch.direction = Direction::EAST;
        ch.current_speed = 100.0;
        ch.request_state(StateRequest::Run);

        let start_y = ch.get_position().y;
        for _ in 0..3 {
            ch.process(0.016, &map);
        }
        let end_y = ch.get_position().y;

        assert!(
            (end_y - start_y).abs() < f32::EPSILON,
            "Y should not change on flat ground"
        );
    }

    // --- reset() tests ---

    #[test]
    fn test_reset_restores_position() {
        let map = make_test_map();
        let mut ch = make_character(0, 0, &map);
        ch.start_cell = Vector2Di::new(1, 0);
        ch.set_cell_position(1, 0); // move to start_cell first so prev_cell is consistent

        // Move to a different cell
        ch.set_cell_position(4, 0);
        assert_eq!(ch.get_cell_position(), Vector2Di::new(4, 0));

        ch.reset();

        assert_eq!(
            ch.get_cell_position(),
            Vector2Di::new(1, 0),
            "position should be restored to start_cell after reset"
        );
    }

    #[test]
    fn test_reset_sets_idle_state() {
        let map = make_test_map();
        let mut ch = make_character(0, 0, &map);
        // Put character into Run state to get it out of the initial None state
        ch.force_transition(StateRequest::Run);
        assert!(
            !ch.is_idle(),
            "character should not be idle after Run transition"
        );

        ch.reset();

        assert!(ch.is_idle(), "character should be idle after reset");
    }

    #[test]
    fn test_reset_clears_speed() {
        let map = make_test_map();
        let mut ch = make_character(0, 0, &map);
        ch.force_transition(StateRequest::Run); // enters Run, sets current_speed = ch.speed
        assert!(
            ch.current_speed > 0.0,
            "speed should be non-zero in Run state"
        );

        ch.reset();

        assert_eq!(ch.current_speed, 0.0, "speed should be 0 after reset");
    }

    #[test]
    fn test_reset_clears_pending_request() {
        let map = make_test_map();
        let mut ch = make_character(0, 0, &map);
        ch.start_cell = Vector2Di::new(0, 0);

        // Queue a Run request without processing it
        ch.request_state(StateRequest::Run);

        ch.reset();

        // After reset the pending request is cleared: processing a tick should
        // keep the character idle rather than switching to Run.
        ch.process(0.016, &map);
        assert!(
            ch.is_idle(),
            "pending request should have been cleared by reset"
        );
    }
}
