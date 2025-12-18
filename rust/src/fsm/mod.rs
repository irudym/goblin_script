use godot::prelude::*;
use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
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
    pub fn to_vector(&self) -> Vector2 {
        use Direction::*;
        match self {
            Self::NORTH => Vector2 { x: 0.0, y: -1.0 },
            Self::EAST => Vector2 { x: 1.0, y: 0.0 },
            Self::SOUTH => Vector2 { x: 0.0, y: 1.0 },
            Self::WEST => Vector2 { x: -1.0, y: 0.0 },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StateRequest {
    Idle,
    Run,
    Turn(Direction),
    WalkTo(Vector2),
}

#[derive(PartialEq, Debug)]
pub enum StateType {
    RUN,
    TURN,
    IDLE,
}

pub trait FSM {
    fn get_type(&self) -> StateType; // return state type
    fn can_transition_to(&self, state_type: StateType) -> bool;

    fn enter(&mut self, character: &mut super::character::Character);
    fn exit(&self, character: &mut super::character::Character);
    fn update(&mut self, delta: f32, character: &mut super::character::Character);

    fn can_exit(&self) -> bool;
}

pub mod idle;
pub mod run;
pub mod turn;
pub mod walk;
