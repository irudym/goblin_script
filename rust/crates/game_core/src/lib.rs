/*
 * Implements pure game logic
 */
#![forbid(unsafe_code)]

pub mod ai;
pub mod api;
pub mod bt;
pub mod character;
pub mod fsm;
pub mod map;

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
