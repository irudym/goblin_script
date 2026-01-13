use crate::bt::blackboard::BlackboardValue;
use crate::bt::result::BTResult;
use crate::bt::{BoxBTNode, NodeStatus};
use crate::character::snapshot::CharacterSnapshot;

use super::BTNode;

/*
 * / Sequence Node - all children must succeed
 */
pub struct Sequence {
    children: Vec<BoxBTNode>,
    id: usize,
}

impl Sequence {
    pub fn new(children: Vec<BoxBTNode>) -> Self {
        Self { children, id: 0 }
    }
}

impl BTNode for Sequence {
    // TODO: add snapshot as a parameter, to get access to Blackboard
    fn reset(&mut self) {
        for child in &mut self.children {
            child.reset();
        }
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn children_mut(&mut self) -> Option<&mut Vec<BoxBTNode>> {
        Some(&mut self.children)
    }

    fn tick(&self, snapshot: &CharacterSnapshot, delta: f32) -> (NodeStatus, BTResult) {
        let mut combined_result = BTResult {
            commands: Vec::new(),
        };

        let bb = &snapshot.blackboard;

        let key = format!("{}.idx", self.id);

        let mut index = match bb.get(&key) {
            Some(BlackboardValue::Int(val)) => val as usize,
            _ => 0,
        };

        while index < self.children.len() {
            let (status, result) = self.children[index].tick(snapshot, delta);

            combined_result.commands.extend(result.commands);

            match status {
                NodeStatus::SUCCESS => index += 1,
                NodeStatus::RUNNING => {
                    bb.set(&key, BlackboardValue::Int(index as i32));
                    return (status, combined_result);
                }
                NodeStatus::FAILURE => {
                    bb.set(&key, BlackboardValue::Int(0));
                    return (status, combined_result);
                }
            }
        }

        // reset index
        bb.set(&key, BlackboardValue::Int(0));
        (NodeStatus::SUCCESS, combined_result)
    }
}

/*
The Selector (OR)
Runs children in order. Succeeds if one succeeds. Fails only if all fail.
This is your "Fallback" logic (e.g., "Try to Attack; if can't, Patrol").
*/

pub struct Selector {
    children: Vec<BoxBTNode>,
    id: usize,
}

impl Selector {
    pub fn new(children: Vec<BoxBTNode>) -> Self {
        Self { children, id: 0 }
    }
}

impl BTNode for Selector {
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn children_mut(&mut self) -> Option<&mut Vec<BoxBTNode>> {
        Some(&mut self.children)
    }

    fn tick(&self, snapshot: &CharacterSnapshot, delta: f32) -> (NodeStatus, BTResult) {
        let bb = &snapshot.blackboard;

        let key = format!("{}.sel", self.id);

        let mut index = match bb.get(&key) {
            Some(BlackboardValue::Int(val)) => val as usize,
            _ => 0,
        };

        while index < self.children.len() {
            let (status, result) = self.children[index].tick(snapshot, delta);

            match status {
                NodeStatus::FAILURE => {
                    index += 1;
                    continue;
                }
                NodeStatus::RUNNING => {
                    bb.set(&key, BlackboardValue::Int(index as i32));
                    return (NodeStatus::RUNNING, result);
                }
                NodeStatus::SUCCESS => {
                    bb.set(&key, BlackboardValue::Int(0));
                    return (NodeStatus::SUCCESS, result);
                }
            }
        }

        // all failed
        bb.set(&key, BlackboardValue::Int(0));
        (NodeStatus::FAILURE, BTResult::empty())
    }

    fn reset(&mut self) {
        //index= 0
        for child in &mut self.children {
            child.reset();
        }
    }
}
