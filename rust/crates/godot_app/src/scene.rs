use game_core::ai::worker::init_bt_system;
use godot::classes::{INode2D, Node2D, TileMapLayer};
use godot::prelude::*;
use platform::logger::LogType;

use crate::godot_logger::GodotLogger;
use game_core::map::GameMap;
use platform::shared::logger_global::log;
use platform::{log, log_debug, log_info};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Scene {
    base: Base<Node2D>,
    game_map: GameMap,
}

#[godot_api]
impl INode2D for Scene {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            game_map: GameMap::new(20, 20),
        }
    }

    fn ready(&mut self) {
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
        //let tilemap = self.base().get_node_as::<TileMapLayer>("TileMapGround");
        //let map = tilemap.get_tile_map_data_as_array();
        //let width = tilemap.
        //

        //log_debug!("Tilemap: {:?}", map);

        let tilemap = self.base().get_node_as::<TileMapLayer>("logic_map");

        let grid_pos = Vector2i::new(10, 10);
        let world_pos = tilemap.map_to_local(grid_pos);
        log_info!("Position: {:?}", world_pos);
    }

    fn process(&mut self, _delta: f32) {
        //self.characters.retain(|c| c.is_instance_valid());

        //for char in self.characters.iter_mut() {
        //let mut character = char.bind_mut();
        //character.update_state(delta);
        //}
    }
}
