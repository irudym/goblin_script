use boa_engine::{
    object::FunctionObjectBuilder, property::Attribute, Context, JsResult, JsString, JsValue,
    NativeFunction,
};

use crate::{api::script_event::ScriptEvent, runtime::script_instance::ScriptInstance};
use game_core::api::commands::PlayerCommand;
use platform::types::Vector2Di;

fn register_function(ctx: &mut Context, name: &str, cmd: PlayerCommand, args: usize) {
    let func = FunctionObjectBuilder::new(ctx.realm(), unsafe {
        NativeFunction::from_closure(move |_this, args, ctx| {
            let instance = ctx
                .get_data::<ScriptInstance>()
                .expect("ScriptInstance missing");

            match cmd {
                PlayerCommand::SetPosition(_) => {
                    if args.len() > 1 {
                        match (args[0].as_i32(), args[1].as_i32()) {
                            (Some(x), Some(y)) => {
                                instance.events.borrow_mut().push(ScriptEvent::Command(
                                    PlayerCommand::SetPosition(Vector2Di { x, y }),
                                ));
                            }
                            (_, _) => (),
                        }
                    }
                }
                _ => {
                    instance.events.borrow_mut().push(ScriptEvent::Command(cmd));
                }
            }
            Ok(JsValue::undefined())
        })
    })
    .name(name)
    .length(args)
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
    register_function(ctx, "step_right", PlayerCommand::MoveEast, 0);
    register_function(ctx, "step_left", PlayerCommand::MoveWest, 0);
    register_function(ctx, "step_up", PlayerCommand::MoveNorth, 0);
    register_function(ctx, "step_down", PlayerCommand::MoveSouth, 0);
    register_function(
        ctx,
        "set_position",
        PlayerCommand::SetPosition(Vector2Di { x: 0, y: 0 }),
        2,
    );

    // Register __line(N) for source line tracking (inserted by preprocessor)
    let line_fn = NativeFunction::from_fn_ptr(step_binding);
    let line_func = FunctionObjectBuilder::new(ctx.realm(), line_fn)
        .name("__line")
        .length(1)
        .build();
    let _ = ctx.register_global_property(JsString::from("__line"), line_func, Attribute::all());
}
