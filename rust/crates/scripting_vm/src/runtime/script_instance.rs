use std::cell::RefCell;

use boa_engine::JsData;
use boa_gc::{Finalize, Trace};
use game_core::api::commands::PlayerCommand;

#[derive(Default, Trace, Finalize, JsData)]
pub struct ScriptInstance {
    #[unsafe_ignore_trace]
    pub commands: RefCell<Vec<PlayerCommand>>,
}
