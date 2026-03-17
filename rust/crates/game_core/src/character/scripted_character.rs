use crate::character::request::StateRequest;
use crate::character::snapshot::CharacterSnapshot;
use crate::character::CharacterId;
use crate::map::LogicMap;
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
}
