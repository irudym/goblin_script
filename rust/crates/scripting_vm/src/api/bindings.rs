use boa_engine::{
    object::FunctionObjectBuilder, property::Attribute, Context, JsResult, JsString, JsValue,
    NativeFunction,
};

use crate::{api::script_event::ScriptEvent, runtime::script_instance::ScriptInstance};
use game_core::api::commands::PlayerCommand;

fn register_function(ctx: &mut Context, name: &str, cmd: PlayerCommand) {
    let func = FunctionObjectBuilder::new(ctx.realm(), unsafe {
        NativeFunction::from_closure(move |_this, _args, ctx| {
            let instance = ctx
                .get_data::<ScriptInstance>()
                .expect("ScriptInstance missing");

            instance.events.borrow_mut().push(ScriptEvent::Command(cmd));

            Ok(JsValue::undefined())
        })
    })
    .name(name)
    .length(0)
    .build();

    let _ = ctx.register_global_property(JsString::from(name), func, Attribute::all());
}

fn step_binding(_this: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let line = args.get(0).and_then(|v| v.as_number()).unwrap_or(0.0) as usize;

    let instance = ctx.get_data::<ScriptInstance>().unwrap();
    instance.events.borrow_mut().push(ScriptEvent::Line(line));

    Ok(JsValue::undefined())
}

pub fn register_api(ctx: &mut Context) {
    register_function(ctx, "step_right", PlayerCommand::MoveEast);
    register_function(ctx, "step_left", PlayerCommand::MoveWest);
    register_function(ctx, "step_up", PlayerCommand::MoveNorth);
    register_function(ctx, "step_down", PlayerCommand::MoveSouth);
}
