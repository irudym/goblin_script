use platform::types::{Direction, Vector2D};

#[derive(Clone, Debug)]
pub struct CharacterSnapshot {
    pub position: Vector2D,
    pub direction: Direction,
    pub velocity: Vector2D,
    pub is_idle: bool,
}
