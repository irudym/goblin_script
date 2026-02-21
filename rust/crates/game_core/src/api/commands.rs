use platform::types::Direction;

#[derive(Debug, Clone, Copy)]
pub enum PlayerCommand {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
    Wait(f32),
    Move(Direction),
}
