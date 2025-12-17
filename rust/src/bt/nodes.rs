use crate::bt::NodeStatus;

use super::BTNode;

/*
 * / Sequence Node - all children must succeed
 */
pub struct Sequence {
    index: usize,
    children: Vec<Box<dyn BTNode>>,
}

impl Sequence {
    pub fn new() -> Self {
        Self {
            index: 0,
            children: Vec::new(),
        }
    }
}

impl BTNode for Sequence {
    fn tick(&mut self, delta: f32) -> NodeStatus {
        for child in self.children.iter_mut() {
            let result = child.tick(delta);
            if result != NodeStatus::SUCCESS {
                return result;
            }
        }
        NodeStatus::SUCCESS
    }
}
