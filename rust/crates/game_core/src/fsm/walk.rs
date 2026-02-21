use crate::character::request::StateRequest;
use crate::fsm::{StateType, FSM};
use crate::CharacterLogic;
use platform::types::Vector2D;

pub struct WalkState {
    target: Vector2D,
    can_exit: bool,
}

impl WalkState {
    pub fn new(target: Vector2D) -> Self {
        Self {
            target,
            can_exit: false,
        }
    }
}

impl FSM for WalkState {
    fn get_type(&self) -> StateType {
        StateType::RUN
    }

    fn can_transition_to(&self, state_type: StateType) -> bool {
        state_type == StateType::IDLE
    }

    fn enter(&mut self, character: &mut CharacterLogic) {
        // get direction from target and current pos
        let current_pos = character.get_position();
        let direction = current_pos.direction_to(self.target);

        if direction != character.direction {
            character.request_state(StateRequest::Idle);
            character.request_state(StateRequest::Turn(direction));
        } else {
            character.set_current_speed(character.speed);
            character.play_animation_with_direction("run");
        }
    }

    fn exit(&self, _character: &mut CharacterLogic) {}

    fn update(&mut self, delta: f32, character: &mut CharacterLogic) {
        let current_pos = character.get_position();

        //use Godot's move toward method
        let new_pos = current_pos.move_toward(self.target, character.speed * delta);
        //character.set_position(new_pos);

        if new_pos.approx_eq(&self.target) {
            self.can_exit = true;
            character.request_state(StateRequest::Idle);
        }
    }

    fn can_exit(&self) -> bool {
        self.can_exit
    }
}
