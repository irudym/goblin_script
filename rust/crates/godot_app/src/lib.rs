mod character;
mod godot_animator;
mod godot_logger;
mod scene;

use game_core::ai::worker::init_bt_system;
use godot::classes::{ITileMapLayer, TileMapLayer};
use godot::prelude::*;

use crate::godot_logger::GodotLogger;
use platform::logger::LogType;
use platform::shared::logger_global::init_logger;
use platform::{log_info, Logger};

struct GoblinExtension;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
struct GroundMap {
    base: Base<TileMapLayer>,
}

#[godot_api]
impl ITileMapLayer for GroundMap {
    fn init(base: Base<TileMapLayer>) -> Self {
        //godot_print!("godot-rust initialized!"); // Prints to the Godot console

        Self { base }
    }

    fn physics_process(&mut self, _delta: f64) {
        // let data = self.base().get_tile_map_data_as_array();
        // godot_print!("MAP data: {:?}", &data);
    }
}

#[gdextension]
unsafe impl ExtensionLibrary for GoblinExtension {
    fn on_stage_init(stage: InitStage) {
        godot_print!("on_stage_init: {:?}", stage);
        match stage {
            InitStage::Scene => {
                godot_print!("godot-rust initialized!");
                godot_print!("Initializing the Logger");

                init_logger(Box::new(GodotLogger) as Box<dyn Logger + Send + Sync>);
                log_info!("Logger is ready!");

                log_info!("Initializing the AI system");
                init_bt_system();
                log_info!("AI System is ready!");
            }
            _ => (),
        }
    }
}
