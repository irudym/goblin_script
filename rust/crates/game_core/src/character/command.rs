use platform::types::{Direction, Vector2D};

use crate::StateRequest;

#[derive(Debug, Clone)]
pub enum CharacterCommand {
    MoveToward(Vector2D),
    SetDirection(Direction),
    PlayAnimation(String),
    ChangeState(StateRequest),
    SnapToCell,
}
