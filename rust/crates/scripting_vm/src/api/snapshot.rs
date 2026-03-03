use boa_engine::{Context, JsObject, JsResult, JsString, JsValue};
use game_core::character::snapshot::CharacterSnapshot;

/* Helper to transform CharacterSnapshot to JSObject
 * Field mapping:
  ┌─────────────────────┬────────────────────────────────┬───────────────────────────────┐
  │     JS property     │          Rust source           │            JS type            │
  ├─────────────────────┼────────────────────────────────┼───────────────────────────────┤
  │ character.id        │ snapshot.id (u32)              │ number                        │
  ├─────────────────────┼────────────────────────────────┼───────────────────────────────┤
  │ character.x         │ snapshot.position.x (f32)      │ number                        │
  ├─────────────────────┼────────────────────────────────┼───────────────────────────────┤
  │ character.y         │ snapshot.position.y (f32)      │ number                        │
  ├─────────────────────┼────────────────────────────────┼───────────────────────────────┤
  │ character.vx        │ snapshot.velocity.x (f32)      │ number                        │
  ├─────────────────────┼────────────────────────────────┼───────────────────────────────┤
  │ character.vy        │ snapshot.velocity.y (f32)      │ number                        │
  ├─────────────────────┼────────────────────────────────┼───────────────────────────────┤
  │ character.speed     │ snapshot.current_speed (f32)   │ number                        │
  ├─────────────────────┼────────────────────────────────┼───────────────────────────────┤
  │ character.is_idle   │ snapshot.is_idle (bool)        │ boolean                       │
  ├─────────────────────┼────────────────────────────────┼───────────────────────────────┤
  │ character.direction │ snapshot.direction.to_string() │ "north"/"south"/"east"/"west" │
  └─────────────────────┴────────────────────────────────┴───────────────────────────────┘

  snapshot.blackboard is omitted
*/
pub fn snapshot_to_js_object(
    snapshot: &CharacterSnapshot,
    ctx: &mut Context,
) -> JsResult<JsObject> {
    let obj = JsObject::with_object_proto(ctx.intrinsics());

    obj.set(
        JsString::from("id"),
        JsValue::from(snapshot.id as f64),
        false,
        ctx,
    )?;
    obj.set(
        JsString::from("x"),
        JsValue::from(snapshot.position.x as f64),
        false,
        ctx,
    )?;
    obj.set(
        JsString::from("y"),
        JsValue::from(snapshot.position.y as f64),
        false,
        ctx,
    )?;
    obj.set(
        JsString::from("vx"),
        JsValue::from(snapshot.velocity.x as f64),
        false,
        ctx,
    )?;
    obj.set(
        JsString::from("vy"),
        JsValue::from(snapshot.velocity.y as f64),
        false,
        ctx,
    )?;
    obj.set(
        JsString::from("speed"),
        JsValue::from(snapshot.current_speed as f64),
        false,
        ctx,
    )?;
    obj.set(
        JsString::from("is_idle"),
        JsValue::from(snapshot.is_idle),
        false,
        ctx,
    )?;
    obj.set(
        JsString::from("direction"),
        JsValue::from(JsString::from(snapshot.direction.to_string().as_str())),
        false,
        ctx,
    )?;

    Ok(obj)
}
