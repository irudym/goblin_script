use crate::StateRequest;
use platform::types::{Direction, Vector2D};

#[derive(Debug, Clone)]
pub enum BTCommand {
    MoveToward(Vector2D),
    SetDirection(Direction),
    PlayAnimation(String),
    ChangeState(StateRequest),
    SnapToCell,
    Custom(String),
}
