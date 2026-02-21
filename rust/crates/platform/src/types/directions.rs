use crate::types::Vector2D;
use std::fmt::Display;

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum Direction {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;
        match self {
            NORTH => write!(f, "north"),
            SOUTH => write!(f, "south"),
            WEST => write!(f, "west"),
            EAST => write!(f, "east"),
        }
    }
}

impl Direction {
    pub fn to_vector(&self) -> Vector2D {
        use Direction::*;
        match self {
            NORTH => Vector2D { x: 0.0, y: -1.0 },
            EAST => Vector2D { x: 1.0, y: 0.0 },
            SOUTH => Vector2D { x: 0.0, y: 1.0 },
            WEST => Vector2D { x: -1.0, y: 0.0 },
        }
    }
}
