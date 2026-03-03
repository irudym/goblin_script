use boa_engine::{Context, JsString, JsValue, Source};

use game_core::api::commands::ExecutionPlayerCommand;
use game_core::character::snapshot::CharacterSnapshot;

use crate::{
    api::{
        bindings::register_api, preprocessor::instrument_code, script_event::ScriptEvent,
        snapshot::snapshot_to_js_object,
    },
    runtime::script_instance::ScriptInstance,
    vm::script_error::ScriptError,
};

const MAX_LOOP_ITERATIONS: u64 = 10_000;

pub struct ScriptVM {
    ctx: Context,
    code: String,
}

impl ScriptVM {
    pub fn new(code: &str) -> Result<Self, ScriptError> {
        let mut ctx = Context::default();

        ctx.runtime_limits_mut()
            .set_loop_iteration_limit(MAX_LOOP_ITERATIONS);

        ctx.insert_data(ScriptInstance::default());

        register_api(&mut ctx);

        Ok(Self {
            ctx,
            code: code.to_string(),
        })
    }

    pub fn set_code(&mut self, code: &str) {
        self.code = code.to_string();
    }

    pub fn run_script(&mut self) -> Result<Vec<ExecutionPlayerCommand>, ScriptError> {
        let instrumented = instrument_code(&self.code);

        let _ = self
            .ctx
            .eval(Source::from_bytes(&instrumented))
            .map_err(ScriptError::from_js_error)?;

        if let Some(instance) = self.ctx.get_data::<ScriptInstance>() {
            let events = std::mem::take(&mut *instance.events.borrow_mut());
            Ok(collapse_events(events))
        } else {
            Ok(vec![])
        }
    }

    pub fn tick(
        &mut self,
        snapshot: &CharacterSnapshot,
    ) -> Result<Vec<ExecutionPlayerCommand>, ScriptError> {
        // Build the character JS object from the snapshot
        let char_obj =
            snapshot_to_js_object(snapshot, &mut self.ctx).map_err(ScriptError::from_js_error)?;

        // Get the 'update' function from global scope
        let global = self.ctx.global_object();
        let update_val = global
            .get(JsString::from("update"), &mut self.ctx)
            .map_err(ScriptError::from_js_error)?;

        // Call update(character) - skip if not defined or not callable
        if let Some(func_obj) = update_val.as_function() {
            func_obj
                .call(&JsValue::undefined(), &[char_obj.into()], &mut self.ctx)
                .map_err(ScriptError::from_js_error)?;
        } else {
            return Ok(vec![]);
        }

        // Collect events
        if let Some(instance) = self.ctx.get_data::<ScriptInstance>() {
            let events = std::mem::take(&mut *instance.events.borrow_mut());
            Ok(collapse_events(events))
        } else {
            Ok(vec![])
        }
    }
}

/// Collapse interleaved ScriptEvents into ExecutionPlayerCommands.
///
/// Events come in pairs: `[Line(2), Command(MoveNorth), Line(4), Command(MoveEast)]`
/// Each `Line(N)` sets the current line, each `Command(cmd)` produces an
/// `ExecutionPlayerCommand` using the most recent line number.
fn collapse_events(events: Vec<ScriptEvent>) -> Vec<ExecutionPlayerCommand> {
    let mut result = Vec::new();
    let mut current_line: usize = 0;

    for event in events {
        match event {
            ScriptEvent::Line(line) => {
                current_line = line;
            }
            ScriptEvent::Command(cmd) => {
                result.push(ExecutionPlayerCommand {
                    command: cmd,
                    line: current_line,
                });
            }
        }
    }

    result
}
