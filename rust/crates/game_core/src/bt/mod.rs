/*
 * Behaviour Tree implementation
 */
pub mod blackboard;
pub mod leafs;
pub mod nodes;

pub use blackboard::Blackboard;

use crate::CharacterLogic;
use platform::{Animator, Logger};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NodeStatus {
    RUNNING,
    SUCCESS,
    FAILURE,
}

pub trait BTNode<A: Animator, L: Logger>: Send + Sync {
    fn reset(&mut self);
    fn tick(
        &mut self,
        context: &mut CharacterLogic<A, L>,
        blackboard: &Blackboard,
        delta: f32,
    ) -> NodeStatus;
}

pub type BoxBTNode<A, L> = Box<dyn BTNode<A, L>>;
