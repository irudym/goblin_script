use boa_engine::{Context, Source};

use game_core::api::commands::PlayerCommand;

use crate::{api::bindings::register_api, runtime::script_instance::ScriptInstance};

pub struct ScriptVM {
    ctx: Context,
}

impl ScriptVM {
    pub fn new(code: &str) -> Self {
        let mut ctx = Context::default();

        ctx.insert_data(ScriptInstance::default());

        register_api(&mut ctx);
        ctx.eval(Source::from_bytes(code)).unwrap();

        Self { ctx }
    }

    pub fn tick(&mut self) -> Vec<PlayerCommand> {
        let _ = self.ctx.eval(Source::from_bytes("update();"));
        if let Some(instance) = self.ctx.get_data::<ScriptInstance>() {
            std::mem::take(&mut *instance.commands.borrow_mut())
        } else {
            vec![]
        }
    }
}
