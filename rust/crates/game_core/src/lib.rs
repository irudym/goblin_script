/*
 * Implements pure game logic
 */
#![forbid(unsafe_code)]

pub mod bt;
pub mod character;
pub mod fsm;
pub mod math;

/*
 * Public re-export
 */
pub use character::character::CharacterLogic;
pub use character::request::StateRequest;

//FSM
pub use fsm::StateType;

//Behaviour Tree
pub use bt::BoxBTNode;
pub use bt::NodeStatus;

//Math
pub use math::vector2d::Vector2D;
