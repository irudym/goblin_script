mod bt;
mod character;
mod fsm;
mod scene;

use godot::classes::{ITileMapLayer, TileMapLayer};
use godot::prelude::*;

struct GoblinExtension;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
struct GroundMap {
    base: Base<TileMapLayer>,
}

#[godot_api]
impl ITileMapLayer for GroundMap {
    fn init(base: Base<TileMapLayer>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self { base }
    }

    fn physics_process(&mut self, _delta: f64) {
        // let data = self.base().get_tile_map_data_as_array();
        // godot_print!("MAP data: {:?}", &data);
    }
}

#[gdextension]
unsafe impl ExtensionLibrary for GoblinExtension {}
