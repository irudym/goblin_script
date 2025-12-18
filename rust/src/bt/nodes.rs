use crate::bt::Blackboard;
use crate::bt::{BoxBTNode, NodeStatus};
use crate::character::Character;

use super::BTNode;

/*
 * / Sequence Node - all children must succeed
 */
pub struct Sequence {
    index: usize,
    children: Vec<BoxBTNode>,
}

impl Sequence {
    pub fn new(children: Vec<BoxBTNode>) -> Self {
        Self { index: 0, children }
    }
}

impl BTNode for Sequence {
    fn reset(&mut self) {
        self.index = 0;
        for child in &mut self.children {
            child.reset();
        }
    }

    fn tick(&mut self, context: &mut Character, blackboard: &Blackboard, delta: f32) -> NodeStatus {
        while self.index < self.children.len() {
            let status = self.children[self.index].tick(context, blackboard, delta);

            match status {
                NodeStatus::RUNNING => {
                    return NodeStatus::RUNNING;
                }
                NodeStatus::FAILURE => {
                    self.index = 0; //reset the index
                    return NodeStatus::FAILURE;
                }
                NodeStatus::SUCCESS => {
                    self.index += 1;
                }
            }
        }

        self.index = 0;
        NodeStatus::SUCCESS
    }
}

/*
The Selector (OR)
Runs children in order. Succeeds if one succeeds. Fails only if all fail.
This is your "Fallback" logic (e.g., "Try to Attack; if can't, Patrol").
*/

pub struct Selector {
    children: Vec<BoxBTNode>,
    index: usize,
}

impl Selector {
    pub fn new(children: Vec<BoxBTNode>) -> Self {
        Self { children, index: 0 }
    }
}

impl BTNode for Selector {
    fn tick(&mut self, context: &mut Character, blackboard: &Blackboard, delta: f32) -> NodeStatus {
        while self.index < self.children.len() {
            let status = self.children[self.index].tick(context, blackboard, delta);

            match status {
                NodeStatus::RUNNING => {
                    return NodeStatus::RUNNING;
                }
                NodeStatus::SUCCESS => {
                    self.index = 0;

                    return NodeStatus::SUCCESS;
                }
                NodeStatus::FAILURE => {
                    self.index += 1;
                }
            }
        }
        self.index = 0;
        NodeStatus::FAILURE
    }

    fn reset(&mut self) {
        self.index = 0;
        for child in &mut self.children {
            child.reset();
        }
    }
}
