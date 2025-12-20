use platform::types::{Direction, Vector2D};

#[derive(Debug, Clone, PartialEq)]
pub enum StateRequest {
    Idle,
    Run,
    Turn(Direction),
    WalkTo(Vector2D),
}
