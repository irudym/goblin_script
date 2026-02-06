use super::{StateType, FSM};
use crate::CharacterLogic;

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

    fn enter(&mut self, character: &mut CharacterLogic) {
        character.play_animation_with_direction("run");
        character.set_current_speed(character.speed);
    }
    fn exit(&self, _character: &mut CharacterLogic) {}

    fn update(&mut self, _delta: f32, _character: &mut CharacterLogic) {
        // move to the direction
        /*
        let current_pos = character.get_position();

        // get direction vector
        let direction_vector = character.direction.to_vector();

        let new_pos = direction_vector * character.speed * delta;
        character.set_position(current_pos + new_pos);
        */
    }

    fn can_exit(&self) -> bool {
        return true;
    }
}
