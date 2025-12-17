use std::fmt::Display;

#[derive(Clone, PartialEq)]
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
