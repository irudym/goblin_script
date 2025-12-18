/*
 * Behaviour Tree implementation
 */
use crate::bt::blackboard::Blackboard;
use crate::character::Character;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NodeStatus {
    RUNNING,
    SUCCESS,
    FAILURE,
}

pub trait BTNode: Send + Sync {
    fn reset(&mut self);
    fn tick(&mut self, context: &mut Character, blackboard: &Blackboard, delta: f32) -> NodeStatus;
}

pub type BoxBTNode = Box<dyn BTNode>;

pub mod blackboard;
pub mod leafs;
pub mod nodes;
