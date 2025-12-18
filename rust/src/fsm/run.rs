use super::{FSM, StateType};
use crate::character::Character;
use godot::prelude::*;

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
    fn update(&mut self, delta: f32, character: &mut Character) {
        // move to the direction
        let current_pos = character.base().get_position();

        // get direction vector
        let direction_vector = character.direction.to_vector();

        //use Godot's move toward method
        //let new_pos = current_pos.move_toward(direction_vector, character.speed * delta);
        let new_pos = direction_vector * character.speed * delta;
        character.base_mut().set_position(current_pos + new_pos);
    }

    fn can_exit(&self) -> bool {
        return true;
    }
}
