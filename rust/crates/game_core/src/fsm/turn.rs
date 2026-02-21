use super::{StateType, FSM};
use crate::CharacterLogic;
use platform::types::Direction;

pub struct TurnState {
    allowed_transition: Vec<StateType>,
    target: Direction,
    can_exit: bool,
}

impl TurnState {
    pub fn new(target: Direction) -> Self {
        Self {
            allowed_transition: vec![StateType::RUN, StateType::TURN, StateType::IDLE],
            can_exit: false,
            target,
        }
    }
}

impl FSM for TurnState {
    fn get_type(&self) -> StateType {
        StateType::TURN
    }

    fn can_transition_to(&self, state_type: StateType) -> bool {
        self.allowed_transition.contains(&state_type)
    }

    fn enter(&mut self, character: &mut CharacterLogic) {
        character.set_current_speed(0.0);
        if self.target == character.direction {
            self.can_exit = true;
            character.request_state(crate::StateRequest::Idle);
        } else {
            //start play turn animation to provided direction
            let animation = format!("turn_{}_{}", character.direction, self.target);
            character.play_animation(&animation);
            character.direction = self.target.clone();
        }
    }

    fn exit(&self, _character: &mut CharacterLogic) {}

    fn update(&mut self, _delta: f32, character: &mut CharacterLogic) {
        // check if the turning animation is playing
        if !character.is_animation_playing() {
            self.can_exit = true;
            character.request_state(crate::StateRequest::Idle);
        }
    }

    fn can_exit(&self) -> bool {
        return self.can_exit;
    }
}
