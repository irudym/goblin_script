use platform::types::Vector2D;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::NodeStatus;

#[derive(Clone, Debug, PartialEq)]
pub enum BlackboardValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Vector(Vector2D),
    NodeState(NodeStatus),
}

#[derive(Clone, Default, Debug)]
pub struct Blackboard {
    data: Arc<RwLock<HashMap<String, BlackboardValue>>>,
}

impl Blackboard {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    //Helper to write data
    pub fn set(&self, key: &str, value: BlackboardValue) {
        if let Ok(mut data) = self.data.write() {
            data.insert(key.to_string(), value);
        }
    }

    // Helper to read data (returns a clone of the value)
    pub fn get(&self, key: &str) -> Option<BlackboardValue> {
        if let Ok(data) = self.data.read() {
            data.get(key).cloned()
        } else {
            None
        }
    }

    // Helper: check if a key exist
    pub fn has(&self, key: &str) -> bool {
        if let Ok(data) = self.data.read() {
            data.contains_key(key)
        } else {
            false
        }
    }

    pub fn get_vector(&self, key: &str) -> Option<Vector2D> {
        let value = self.get(key)?;

        match value {
            BlackboardValue::Vector(vector) => Some(vector),
            _ => None,
        }
    }
}
