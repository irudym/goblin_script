use crate::bt::result::BTResult;
use crate::bt::{blackboard::BlackboardValue, BTNode, NodeStatus};
use crate::character::snapshot::CharacterSnapshot;

pub struct Wait {
    delay: f32,
    id: usize,
}

impl Wait {
    pub fn new(delay: f32) -> Self {
        Self { delay, id: 0 }
    }
}

impl BTNode for Wait {
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn id(&self) -> usize {
        self.id
    }

    fn reset(&mut self) {}

    fn tick(&self, snapshot: &CharacterSnapshot, delta: f32) -> (NodeStatus, BTResult) {
        let bb = &snapshot.blackboard;
        let key = format!("{}.timer", self.id);

        let mut current_time = match bb.get(&key) {
            Some(BlackboardValue::Float(val)) => val,
            _ => 0.0,
        };

        current_time += delta;

        if current_time > self.delay {
            bb.set(&key, BlackboardValue::Float(0.0));
            return (NodeStatus::SUCCESS, BTResult::empty());
        }

        bb.set(&key, BlackboardValue::Float(current_time));
        (NodeStatus::RUNNING, BTResult::empty())
    }
}
