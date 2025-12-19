use crate::bt::Blackboard;
use crate::bt::{BoxBTNode, NodeStatus};
use crate::CharacterLogic;
use platform::{Animator, Logger};

use super::BTNode;

/*
 * / Sequence Node - all children must succeed
 */
pub struct Sequence<A: Animator, L: Logger> {
    index: usize,
    children: Vec<BoxBTNode<A, L>>,
}

impl<A: Animator, L: Logger> Sequence<A, L> {
    pub fn new(children: Vec<BoxBTNode<A, L>>) -> Self {
        Self { index: 0, children }
    }
}

impl<A: Animator, L: Logger> BTNode<A, L> for Sequence<A, L> {
    fn reset(&mut self) {
        self.index = 0;
        for child in &mut self.children {
            child.reset();
        }
    }

    fn tick(
        &mut self,
        context: &mut CharacterLogic<A, L>,
        blackboard: &Blackboard,
        delta: f32,
    ) -> NodeStatus {
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

pub struct Selector<A: Animator, L: Logger> {
    children: Vec<BoxBTNode<A, L>>,
    index: usize,
}

impl<A: Animator, L: Logger> Selector<A, L> {
    pub fn new(children: Vec<BoxBTNode<A, L>>) -> Self {
        Self { children, index: 0 }
    }
}

impl<A: Animator, L: Logger> BTNode<A, L> for Selector<A, L> {
    fn tick(
        &mut self,
        context: &mut CharacterLogic<A, L>,
        blackboard: &Blackboard,
        delta: f32,
    ) -> NodeStatus {
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
