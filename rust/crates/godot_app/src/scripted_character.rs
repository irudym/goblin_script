use game_core::character::CharacterId;
use game_core::map::LogicMap;
use godot::classes::{AnimatedSprite2D, Area2D, IArea2D};
use godot::prelude::*;
use platform::logger::LogType;
use platform::types::Vector2D;

use crate::godot_animator::GodotAnimator;
use game_core::CharacterLogic;

use platform::{log_error, log_info};
use std::sync::Arc;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct ScriptedCharacter {
    base: Base<Area2D>,
    pub logic: Option<CharacterLogic>,
    logic_map: Option<Arc<LogicMap>>,
}

impl ScriptedCharacter {
    //#[func]
    pub fn set_logic_map(&mut self, logic_map: Arc<LogicMap>) {
        self.logic_map = Some(logic_map);
    }

    fn get_id(&self) -> CharacterId {
        let variant = self.base().get_meta("id");
        let generated_id = self.base().get_name().hash_u32();

        if variant.is_nil() {
            return generated_id;
        }

        if let Ok(id) = variant.try_to::<i32>() {
            id as u32
        } else {
            log_error!("Meta ID in not an Int");
            generated_id
        }
    }
}

#[godot_api]
impl IArea2D for ScriptedCharacter {
    fn init(base: Base<Area2D>) -> Self {
        ScriptedCharacter {
            base,
            logic: None,
            logic_map: None,
        }
    }

    fn ready(&mut self) {
        let name = self.base().get_name();
        log_info!("Character {} loaded", &name);

        let sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        let position = sprite.get_position();
        let animator = Box::new(GodotAnimator::new(sprite));

        //get id from meta
        let id = self.get_id();
        log_info!("Character[{}] id: {}", &name, id);

        let mut logic = CharacterLogic::new(id, animator);

        logic.set_position(Vector2D {
            x: position.x,
            y: position.y,
        });
        logic.start_cell = logic.snap_to_cell();

        self.logic = Some(logic);
    }

    fn process(&mut self, delta: f32) {
        if let Some(logic) = &mut self.logic {
            if let Some(logic_map) = &mut self.logic_map {
                logic.process(delta, &logic_map);
            }
        }
    }
}
