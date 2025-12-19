use crate::CharacterLogic;
use platform::{Animator, Logger};

#[derive(PartialEq, Debug)]
pub enum StateType {
    RUN,
    TURN,
    IDLE,
}

pub trait FSM<A: Animator, L: Logger> {
    fn get_type(&self) -> StateType; // return state type
    fn can_transition_to(&self, state_type: StateType) -> bool;

    fn enter(&mut self, character: &mut CharacterLogic<A, L>);
    fn exit(&self, character: &mut CharacterLogic<A, L>);
    fn update(&mut self, delta: f32, character: &mut CharacterLogic<A, L>);

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
