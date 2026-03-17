/*
 * Implements pure game logic
 */
#![forbid(unsafe_code)]

pub mod ai;
pub mod api;
pub mod bt;
pub mod character;
pub mod executor;
pub mod fsm;
pub mod map;
pub mod test_utils;

/*
 * Public re-export
 */
pub use character::character::CharacterLogic;
pub use character::npc_character::NPCCharacterLogic;
pub use character::request::StateRequest;
pub use character::scripted_character::ScriptedCharacterLogic;

//FSM
pub use fsm::StateType;

//Behaviour Tree
pub use bt::BoxBTNode;
pub use bt::NodeStatus;

//CommandExecutor
pub use executor::command_executor::CommandExecutor;
