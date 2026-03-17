use platform::log_debug;
use platform::logger::LogType;

use super::{StateType, FSM};
use crate::character::request::StateRequest;
use crate::CharacterLogic;

pub struct WaitState {
    remaining_ms: f32,
    can_exit: bool,
}

impl WaitState {
    pub fn new(duration_ms: f32) -> Self {
        Self {
            remaining_ms: duration_ms,
            can_exit: false,
        }
    }
}

impl FSM for WaitState {
    fn get_type(&self) -> StateType {
        StateType::WAIT
    }

    fn can_transition_to(&self, state_type: StateType) -> bool {
        state_type == StateType::IDLE
    }

    fn enter(&mut self, character: &mut CharacterLogic) {
        character.play_animation_with_direction("stand");
        character.set_current_speed(0.0);
        log_debug!("[WaitState]: enter, delay: {}", self.remaining_ms);
    }

    fn exit(&self, _character: &mut CharacterLogic) {}

    fn update(&mut self, delta: f32, character: &mut CharacterLogic) {
        self.remaining_ms -= delta;
        log_debug!(
            "[WaitState]: delta: {}, remained: {}",
            delta,
            self.remaining_ms
        );

        if self.remaining_ms <= 0.0 {
            log_debug!("[WaitState]: finished with remained: {}", self.remaining_ms);
            self.can_exit = true;
            character.request_state(StateRequest::Idle);
        }
    }

    fn can_exit(&self) -> bool {
        self.can_exit
    }
}
