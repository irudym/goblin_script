use crate::CharacterLogic;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum StateType {
    RUN,
    TURN,
    IDLE,
}

pub trait FSM {
    fn get_type(&self) -> StateType; // return state type
    fn can_transition_to(&self, state_type: StateType) -> bool;

    fn enter(&mut self, character: &mut CharacterLogic);
    fn exit(&self, character: &mut CharacterLogic);
    fn update(&mut self, delta: f32, character: &mut CharacterLogic);

    fn can_exit(&self) -> bool;
}

pub mod idle;
pub mod run;
pub mod turn;
pub mod walk;

pub use idle::IdleState;
pub use run::RunState;
pub use turn::TurnState;
pub use walk::WalkState;
