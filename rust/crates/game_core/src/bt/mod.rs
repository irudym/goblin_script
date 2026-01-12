/*
 * Behaviour Tree implementation
 */
pub mod blackboard;
pub mod command;
pub mod job;
pub mod leafs;
pub mod nodes;
pub mod result;
pub mod tree;

pub use blackboard::Blackboard;

use crate::{
    bt::{
        leafs::{FindTarget, MoveToTarget},
        nodes::Selector,
        result::BTResult,
    },
    character::snapshot::CharacterSnapshot,
};

pub use tree::{BTRef, BehaviourTree};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NodeStatus {
    RUNNING,
    SUCCESS,
    FAILURE,
}

use std::sync::Arc;

/*
 * Thread-Safe Behaviour Node
 */
pub trait BTNode: Send + Sync {
    fn reset(&mut self);
    fn tick(&self, snapshot: &CharacterSnapshot, delta: f32) -> (NodeStatus, BTResult);

    fn set_id(&mut self, id: usize);
    fn id(&self) -> usize;

    fn children_mut(&mut self) -> Option<&mut Vec<BoxBTNode>> {
        None
    }
}

pub type BoxBTNode = Box<dyn BTNode>;

pub fn build_default_tree() -> BTRef {
    Arc::new(BehaviourTree {
        root: Box::new(Selector::new(vec![
            Box::new(FindTarget::new("target_pos")),
            Box::new(MoveToTarget::new("target_pos")),
        ])),
    })
}
