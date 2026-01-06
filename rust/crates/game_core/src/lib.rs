/*
 * Implements pure game logic
 */
#![forbid(unsafe_code)]

pub mod ai;
pub mod bt;
pub mod character;
pub mod fsm;

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
