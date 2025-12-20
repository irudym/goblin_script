use super::{StateType, FSM};
use crate::CharacterLogic;

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

    fn enter(&mut self, character: &mut CharacterLogic) {
        //godot_print!("Enter to IDLE state");
        character.play_animation_with_direction("stand");
    }

    fn exit(&self, _character: &mut CharacterLogic) {}
    fn update(&mut self, _delta: f32, _character: &mut CharacterLogic) {}

    fn can_exit(&self) -> bool {
        return true;
    }
}
