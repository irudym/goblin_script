#[derive(Debug, Clone, Copy)]
pub enum PlayerCommand {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
    Wait,
}
