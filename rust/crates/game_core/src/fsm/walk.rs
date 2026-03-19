use crate::character::request::StateRequest;
use crate::fsm::{StateType, FSM};
use crate::CharacterLogic;
use platform::types::Vector2D;

pub struct WalkState {
    target: Vector2D,
    initial_dir: Vector2D, // normalized direction from start to target; set in enter()
    can_exit: bool,
}

impl WalkState {
    pub fn new(target: Vector2D) -> Self {
        Self {
            target,
            initial_dir: Vector2D::new(0.0, 0.0),
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
        let diff = self.target - character.get_position();
        let len = diff.length();
        if len > f32::EPSILON {
            self.initial_dir = diff * (1.0 / len);
            character.set_current_speed(character.speed); // enables get_effective_velocity in process()
            character.play_animation_with_direction("run");
        } else {
            // Already at target — exit immediately next update
            self.can_exit = true;
            character.request_state(StateRequest::Idle);
        }
    }

    fn exit(&self, _character: &mut CharacterLogic) {}

    fn update(&mut self, _delta: f32, character: &mut CharacterLogic) {
        // process() has already moved the character this frame via get_effective_velocity().
        // Check if we have reached or passed the target depth along the movement direction.
        let to_target = self.target - character.get_position();
        let dot = to_target.x * self.initial_dir.x + to_target.y * self.initial_dir.y;

        if dot <= 0.0 {
            // Snap only along initial_dir; the perpendicular component (stair Y-drift) is preserved.
            // Formula: current_pos + initial_dir * dot  →  moves backward by |dot| along movement axis.
            let snapped = character.get_position() + self.initial_dir * dot;
            character.set_position(snapped);
            character.set_current_speed(0.0);
            self.can_exit = true;
            character.request_state(StateRequest::Idle);
        }
    }

    fn can_exit(&self) -> bool {
        self.can_exit
    }
}
