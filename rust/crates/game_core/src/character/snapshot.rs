use crate::bt::Blackboard;
use platform::types::{Direction, Vector2D};

pub type CharacterId = u64;

#[derive(Clone, Debug)]
pub struct CharacterSnapshot {
    pub id: CharacterId,
    pub position: Vector2D,
    pub direction: Direction,
    pub velocity: Vector2D,
    pub is_idle: bool,
    pub blackboard: Box<Blackboard>,
}
