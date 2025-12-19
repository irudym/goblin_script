use crate::math::{Direction, Vector2D};

#[derive(Debug, Clone, PartialEq)]
pub enum StateRequest {
    Idle,
    Run,
    Turn(Direction),
    WalkTo(Vector2D),
}
