use crate::character::request::StateRequest;
use crate::fsm::{StateType, FSM};
use crate::math::Vector2D;
use crate::CharacterLogic;

pub struct WalkState {
    target: Vector2D,
    speed: f32,
    can_exit: bool,
}

impl WalkState {
    pub fn new(target: Vector2D) -> Self {
        Self {
            target,
            speed: 100.0,
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
        //check direction

        character.play_animation_with_direction("run");
    }

    fn exit(&self, _character: &mut CharacterLogic) {}

    fn update(&mut self, delta: f32, character: &mut CharacterLogic) {
        let current_pos = character.get_position();

        //use Godot's move toward method
        let new_pos = current_pos.move_toward(self.target, self.speed * delta);
        character.set_position(new_pos);

        if new_pos.approx_eq(&self.target) {
            self.can_exit = true;
            character.request_state(StateRequest::Idle);
        }
    }

    fn can_exit(&self) -> bool {
        self.can_exit
    }
}
