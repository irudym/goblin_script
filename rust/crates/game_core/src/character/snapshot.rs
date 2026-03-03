use crate::bt::Blackboard;
use platform::types::{Direction, Vector2D, Vector2Di};

pub type CharacterId = u32;

#[derive(Clone, Debug)]
pub struct CharacterSnapshot {
    pub id: CharacterId,
    pub position: Vector2D,
    pub cell_position: Vector2Di,
    pub direction: Direction,
    pub velocity: Vector2D,
    pub is_idle: bool,
    pub blackboard: Box<Blackboard>,
    pub current_speed: f32,
}
