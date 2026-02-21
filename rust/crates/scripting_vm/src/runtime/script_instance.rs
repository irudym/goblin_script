use std::cell::RefCell;

use boa_engine::JsData;
use boa_gc::{Finalize, Trace};

use crate::api::script_event::ScriptEvent;

#[derive(Default, Trace, Finalize, JsData)]
pub struct ScriptInstance {
    #[unsafe_ignore_trace]
    pub events: RefCell<Vec<ScriptEvent>>,
}
