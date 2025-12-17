use godot::global::godot_print;

use super::{FSM, StateType};
use crate::character::Character;

pub struct IdleState {
    allowed_transition: Vec<StateType>,
}

impl IdleState {
    pub fn new() -> Self {
        Self {
            allowed_transition: vec![StateType::RUN, StateType::TURN],
        }
    }
}

impl FSM for IdleState {
    fn get_type(&self) -> StateType {
        StateType::IDLE
    }

    fn can_transition_to(&self, state_type: StateType) -> bool {
        self.allowed_transition.contains(&state_type)
    }

    fn enter(&mut self, character: &mut Character) {
        godot_print!("Enter to IDLE state");
        character.play_animation_with_direction("stand");
    }

    fn exit(&self, _character: &mut Character) {}
    fn update(&mut self, _delta: f32, _character: &mut Character) {}

    fn can_exit(&self) -> bool {
        return true;
    }
}
