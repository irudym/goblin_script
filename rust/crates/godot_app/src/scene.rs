use godot::classes::{INode2D, Node2D};
use godot::prelude::*;
use platform::logger::LogType;

use crate::godot_logger::GodotLogger;
use platform::shared::logger_global::{init_logger, log};
use platform::{log, log_debug, log_info};

use game_core::ai::worker::init_bt_system;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Scene {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Scene {
    fn init(base: Base<Node2D>) -> Self {
        init_logger(Box::new(GodotLogger));
        Self { base }
    }

    fn ready(&mut self) {
        init_bt_system();

        // get list of children
        /*
        let children = self.base().get_children();
        log_info!("Children of the scene: {:?}", children);
        for node in children.iter_shared() {
            log_info!("==>> Child: {}", &node.get_name());
            log_info!("==>> Child type: {}", &node.get_class());
            /*
            if let Ok(character) = node.try_cast::<Character>() {

            }
            */
        }
        */
    }

    fn process(&mut self, _delta: f32) {
        //self.characters.retain(|c| c.is_instance_valid());

        //for char in self.characters.iter_mut() {
        //let mut character = char.bind_mut();
        //character.update_state(delta);
        //}
    }
}
