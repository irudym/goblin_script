use boa_engine::{Context, Source};

use game_core::api::commands::PlayerCommand;

use crate::{
    api::bindings::register_api, runtime::script_instance::ScriptInstance,
    vm::script_error::ScriptError,
};

const MAX_LOOP_ITERATIONS: u64 = 10_000;

pub struct ScriptVM {
    ctx: Context,
}

impl ScriptVM {
    pub fn new(code: &str) -> Result<Self, ScriptError> {
        let mut ctx = Context::default();

        ctx.runtime_limits_mut()
            .set_loop_iteration_limit(MAX_LOOP_ITERATIONS);

        ctx.insert_data(ScriptInstance::default());

        register_api(&mut ctx);
        ctx.eval(Source::from_bytes(code))
            .map_err(ScriptError::from_js_error)?;

        Ok(Self { ctx })
    }

    pub fn tick(&mut self) -> Result<Vec<PlayerCommand>, ScriptError> {
        let _ = self
            .ctx
            .eval(Source::from_bytes("update();"))
            .map_err(ScriptError::from_js_error)?;

        if let Some(instance) = self.ctx.get_data::<ScriptInstance>() {
            Ok(std::mem::take(&mut *instance.commands.borrow_mut()))
        } else {
            Ok(vec![])
        }
    }
}
