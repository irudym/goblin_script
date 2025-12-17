/*
 * Behaviour Tree implementation
 */

#[derive(PartialEq, Debug)]
pub enum NodeStatus {
    RUNNING,
    SUCCESS,
    FAILURE,
}

trait BTNode {
    fn tick(&mut self, delta: f32) -> NodeStatus;
}

pub mod nodes;
