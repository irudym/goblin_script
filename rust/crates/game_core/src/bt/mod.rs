/*
 * Behaviour Tree implementation
 */
pub mod blackboard;
pub mod leafs;
pub mod nodes;

pub use blackboard::Blackboard;

use crate::CharacterLogic;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NodeStatus {
    RUNNING,
    SUCCESS,
    FAILURE,
}

pub trait BTNode: Send + Sync {
    fn reset(&mut self);
    fn tick(
        &mut self,
        context: &mut CharacterLogic,
        blackboard: &Blackboard,
        delta: f32,
    ) -> NodeStatus;
}

pub type BoxBTNode = Box<dyn BTNode>;
