use godot::classes::{INode2D, Node2D, TileMapLayer};
use godot::prelude::*;
use platform::logger::LogType;

use crate::character::Character;
use crate::debug_overlay::DebugOverlay;
use crate::godot_logger::GodotLogger;
use crate::grid_overlay::GridOverlay;
use godot::classes::Input;
use platform::shared::logger_global::log;
use platform::{log, log_debug, log_info};
use std::sync::Arc;

use game_core::map::{LogicCell, LogicMap};

fn read_logic_cell(tilemap: &TileMapLayer, cell: Vector2i) -> Option<LogicCell> {
    if let Some(tile_data) = tilemap.get_cell_tile_data(cell) {
        Some(LogicCell {
            walkable: tile_data.get_custom_data("walkable").to::<bool>(),
            height: tile_data.get_custom_data("height").to::<i32>(),
            is_step: tile_data.get_custom_data("step").to::<bool>(),
        })
    } else {
        None
    }
}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Scene {
    base: Base<Node2D>,
    logic_map: Option<Arc<LogicMap>>,
}

#[godot_api]
impl INode2D for Scene {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            logic_map: None,
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
        let logic_tilemap = self.base().get_node_as::<TileMapLayer>("logic_map");

        let used_rect = logic_tilemap.get_used_rect();
        let width = used_rect.size.x as usize;
        let height = used_rect.size.y as usize;

        let origin_x = used_rect.position.x;
        let origin_y = used_rect.position.y;

        let mut logic_map = LogicMap::new(width, height);

        log_debug!(
            "Logic Map len: {}, width: {}, height: {}",
            logic_map.get_data_len(),
            width,
            height
        );

        let coord = logic_tilemap.map_to_local(Vector2i { x: 4, y: 4 });
        let g_coord = logic_tilemap.to_global(coord);

        log_debug!("Test coordinates: (4,4) -> {:?}", g_coord);

        for y in 0..height {
            for x in 0..width {
                let cell = Vector2i::new(origin_x + x as i32, origin_y + y as i32);
                let tile = read_logic_cell(&logic_tilemap, cell);

                logic_map.set_cell(x, y, tile);
            }
        }

        let logic_arc = Arc::new(logic_map);
        self.logic_map = Some(logic_arc.clone());

        // update logic map in Character
        // get SortingNode2D, as it keeps all characters
        let sorting_node = self.base().get_node_as::<Node2D>("SortingNode2D");
        let children = sorting_node.get_children();
        for node in children.iter_shared() {
            log_info!("==>> Child: {}", &node.get_name());
            log_info!("==>> Child type: {}", &node.get_class());

            if let Ok(mut character) = node.try_cast::<Character>() {
                character.bind_mut().set_logic_map(logic_arc.clone());
            }
        }

        let mut overlay = self.base().get_node_as::<DebugOverlay>("DebugOverlay");
        overlay.bind_mut().set_logic_map(logic_arc.clone());

        let mut grid_overlay = self.base().get_node_as::<GridOverlay>("GridOverlay");
        grid_overlay.bind_mut().set_logic_map(logic_arc.clone());

        //let tilemap = self.base().get_node_as::<TileMapLayer>("logic_map");
    }

    fn process(&mut self, _delta: f32) {
        //self.characters.retain(|c| c.is_instance_valid());

        //for char in self.characters.iter_mut() {
        //let mut character = char.bind_mut();
        //character.update_state(delta);
        //}
        // check collision
        let input = Input::singleton();

        if input.is_action_just_pressed("toggle_debug_overlay") {
            let mut overlay = self.base().get_node_as::<DebugOverlay>("DebugOverlay");
            overlay.bind_mut().toggle();
        }

        if input.is_action_just_pressed("toggle_grid_overlay") {
            let mut overlay = self.base().get_node_as::<GridOverlay>("GridOverlay");
            overlay.bind_mut().toggle();
        }
    }
}
