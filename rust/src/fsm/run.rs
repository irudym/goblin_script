use super::{FSM, StateType};
use crate::character::Character;

pub struct RunState {
    // target: Vector2,
    allowed_transition: Vec<StateType>,
}

impl RunState {
    pub fn new(/*target: Vector2*/) -> Self {
        Self {
            // target,
            allowed_transition: vec![StateType::IDLE],
        }
    }
}

impl FSM for RunState {
    fn get_type(&self) -> StateType {
        StateType::RUN
    }

    fn can_transition_to(&self, state_type: StateType) -> bool {
        self.allowed_transition.contains(&state_type)
    }

    fn enter(&mut self, character: &mut Character) {
        character.play_animation_with_direction("run");
    }
    fn exit(&self, _character: &mut Character) {}
    fn update(&mut self, _delta: f32, _character: &mut Character) {}

    fn can_exit(&self) -> bool {
        return true;
    }
}
