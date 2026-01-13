use super::BoxBTNode;
use std::sync::Arc;

use super::result::BTResult;
use crate::bt::leafs::Wait;
use crate::bt::nodes::Sequence;
use crate::character::snapshot::CharacterSnapshot;

pub struct BehaviourTree {
    pub root: BoxBTNode,
}

pub type BTRef = Arc<BehaviourTree>;

impl BehaviourTree {
    pub fn tick(&self, snapshot: &CharacterSnapshot, delta: f32) -> BTResult {
        let (_status, result) = self.root.tick(snapshot, delta);
        result
    }

    pub fn default() -> Self {
        BehaviourTree {
            root: Box::new(Sequence::new(vec![Box::new(Wait::new(1.0))])),
        }
    }

    pub fn new(mut root: BoxBTNode) -> Self {
        let mut counter = 1;

        BehaviourTree::assigns_ids(&mut root, &mut counter);

        BehaviourTree { root }
    }

    fn assigns_ids(node: &mut BoxBTNode, counter: &mut usize) {
        node.set_id(*counter);
        *counter += 1;

        if let Some(children) = node.children_mut() {
            for child in children {
                BehaviourTree::assigns_ids(child, counter);
            }
        }
    }
}
