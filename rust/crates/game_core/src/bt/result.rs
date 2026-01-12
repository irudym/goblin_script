use crate::bt::command::BTCommand;

#[derive(Debug, Clone)]
pub struct BTResult {
    pub commands: Vec<BTCommand>,
}

impl BTResult {
    pub fn empty() -> Self {
        BTResult { commands: vec![] }
    }
}
