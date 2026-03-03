use platform::types::{Direction, Vector2Di};

#[derive(Debug, Clone, Copy)]
pub struct ExecutionPlayerCommand {
    pub command: PlayerCommand,
    pub line: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerCommand {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
    Wait(f32),
    Move(Direction),
    SetPosition(Vector2Di),
}

impl PlayerCommand {
    pub fn get_command_direction(&self) -> Option<Direction> {
        match self {
            PlayerCommand::MoveNorth => Some(Direction::NORTH),
            PlayerCommand::MoveSouth => Some(Direction::SOUTH),
            PlayerCommand::MoveEast => Some(Direction::EAST),
            PlayerCommand::MoveWest => Some(Direction::WEST),
            PlayerCommand::Move(direction) => Some(*direction),
            _ => None,
        }
    }
}
