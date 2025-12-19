use crate::math::Vector2D;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug, PartialEq)]
pub enum BlackboardValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Vector(Vector2D), //Godot Vector2
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
}
