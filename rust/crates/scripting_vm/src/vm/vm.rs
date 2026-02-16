use boa_engine::{Context, Source};

use game_core::api::commands::PlayerCommand;

use crate::{api::bindings::register_api, runtime::script_instance::ScriptInstance};

const MAX_LOOP_ITERATIONS: u64 = 10_000;

pub struct ScriptVM {
    ctx: Context,
}

impl ScriptVM {
    pub fn new(code: &str) -> Self {
        let mut ctx = Context::default();

        ctx.runtime_limits_mut()
            .set_loop_iteration_limit(MAX_LOOP_ITERATIONS);

        ctx.insert_data(ScriptInstance::default());

        register_api(&mut ctx);
        ctx.eval(Source::from_bytes(code)).unwrap();

        Self { ctx }
    }

    pub fn tick(&mut self) -> Vec<PlayerCommand> {
        if let Err(err) = self.ctx.eval(Source::from_bytes("update();")) {
            eprintln!("JS error: {err}");
            return vec![];
        }

        if let Some(instance) = self.ctx.get_data::<ScriptInstance>() {
            std::mem::take(&mut *instance.commands.borrow_mut())
        } else {
            vec![]
        }
    }
}
