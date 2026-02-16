use boa_engine::{
    object::FunctionObjectBuilder, property::Attribute, Context, JsString, JsValue, NativeFunction,
};

use crate::runtime::script_instance::ScriptInstance;
use game_core::api::commands::PlayerCommand;

fn register_function(ctx: &mut Context, name: &str, cmd: PlayerCommand) {
    let func = FunctionObjectBuilder::new(ctx.realm(), unsafe {
        NativeFunction::from_closure(move |_this, _args, ctx| {
            let instance = ctx
                .get_data::<ScriptInstance>()
                .expect("ScriptInstance missing");

            instance.commands.borrow_mut().push(cmd);

            Ok(JsValue::undefined())
        })
    })
    .name(name)
    .length(0)
    .build();

    let _ = ctx.register_global_property(JsString::from(name), func, Attribute::all());
}

pub fn register_api(ctx: &mut Context) {
    register_function(ctx, "step_right", PlayerCommand::MoveEast);
    register_function(ctx, "step_left", PlayerCommand::MoveWest);
    register_function(ctx, "step_up", PlayerCommand::MoveNorth);
    register_function(ctx, "step_down", PlayerCommand::MoveSouth);
}
