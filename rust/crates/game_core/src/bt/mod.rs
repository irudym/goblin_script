/*
 * Behaviour Tree implementation
 */
pub mod blackboard;
pub mod leafs;
pub mod nodes;

pub use blackboard::Blackboard;

use crate::character::{command::CharacterCommand, snapshot::CharacterSnapshot};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NodeStatus {
    RUNNING,
    SUCCESS,
    FAILURE,
}

/*
 * Thread-Safe Behaviour Node
 */
pub trait BTNode: Send + Sync {
    fn reset(&mut self);
    fn tick(
        &mut self,
        snapshot: &CharacterSnapshot,
        blackboard: &Blackboard,
        delta: f32,
        out_commands: &mut Vec<CharacterCommand>,
    ) -> NodeStatus;
}

pub type BoxBTNode = Box<dyn BTNode>;
