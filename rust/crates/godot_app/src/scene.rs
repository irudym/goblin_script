use game_core::executor::ExecutorResult;
use game_core::CommandExecutor;
use godot::classes::{CodeEdit, INode2D, Node2D, RichTextLabel, TextureButton, TileMapLayer};
use godot::prelude::*;
use platform::logger::LogType;

use crate::character::Character;
use crate::debug_overlay::DebugOverlay;
use crate::grid_overlay::GridOverlay;
use crate::scripted_character::ScriptedCharacter;
use godot::classes::Input;
use platform::{log_debug, log_info};
use scripting_vm::ScriptVM;
use std::sync::Arc;

use game_core::map::{LogicCell, LogicMap, StepType};

fn get_step_type(step_type: &str) -> StepType {
    match step_type {
        "left" => StepType::Left,
        "right" => StepType::Right,
        _ => StepType::None,
    }
}

fn read_logic_cell(tilemap: &TileMapLayer, cell: Vector2i) -> Option<LogicCell> {
    if let Some(tile_data) = tilemap.get_cell_tile_data(cell) {
        Some(LogicCell {
            walkable: tile_data.get_custom_data("walkable").to::<bool>(),
            height: tile_data.get_custom_data("height").to::<i32>(),
            step_type: get_step_type(&(tile_data.get_custom_data("step_type").to::<String>())),
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
    code_editor: Option<Gd<CodeEdit>>,
    scripted_character: Option<Gd<ScriptedCharacter>>,
    executor: CommandExecutor,
    log_box: Option<Gd<RichTextLabel>>,
}

#[godot_api]
impl Scene {
    #[func]
    fn on_run_pressed(&mut self) {
        log_debug!("Run button pressed!");
        if let Some(log_box) = &mut self.log_box {
            log_box.set_text("");
        }

        if let Some(editor) = &self.code_editor {
            let text = editor.get_text();
            self.run_script(text.to_string());
        }
    }

    fn run_script(&mut self, code: String) {
        log_debug!("Run script: {}", &code);

        match ScriptVM::new(&code) {
            Ok(mut vm) => match vm.run_script() {
                Ok(commands) => {
                    log_debug!("Commands: {:?}", commands);
                    self.executor.set_commands(commands);
                }
                Err(e) => {
                    log_debug!("Script error: {:?}", e);
                    if let Some(log_box) = &mut self.log_box {
                        log_box.set_text(&e.message);
                    }
                }
            },
            Err(e) => {
                log_debug!("Failed to create VM: {:?}", e);
            }
        }
    }
}

#[godot_api]
impl INode2D for Scene {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            logic_map: None,
            code_editor: None,
            scripted_character: None,
            executor: CommandExecutor::new(),
            log_box: None,
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

        let _ = logic_map.save_to_file("logic_map.ron");

        let logic_arc = Arc::new(logic_map);
        self.logic_map = Some(logic_arc.clone());

        // update logic map in Characters
        // get SortingNode2D, as it keeps all characters
        let sorting_node = self.base().get_node_as::<Node2D>("SortingNode2D");
        let children = sorting_node.get_children();
        for node in children.iter_shared() {
            log_info!("==>> Child: {}", &node.get_name());
            log_info!("==>> Child type: {}", &node.get_class());

            if let Ok(mut character) = node.clone().try_cast::<Character>() {
                character.bind_mut().set_logic_map(logic_arc.clone());
            }

            // update the scripted character as well
            if let Ok(mut character) = node.try_cast::<ScriptedCharacter>() {
                character.bind_mut().set_logic_map(logic_arc.clone());
                self.scripted_character = Some(character.clone());
            }
        }

        let mut overlay = self.base().get_node_as::<DebugOverlay>("DebugOverlay");
        overlay.bind_mut().set_logic_map(logic_arc.clone());

        let mut grid_overlay = self.base().get_node_as::<GridOverlay>("GridOverlay");
        grid_overlay.bind_mut().set_logic_map(logic_arc.clone());

        // Get CodeEdit
        log_debug!("CodeEditor");
        let editor = self.base().get_node_as::<CodeEdit>("CodeEdit");
        self.code_editor = Some(editor);

        log_debug!("CodeEditor loaded: {:?}", &self.code_editor);

        let log_box = self.base().get_node_as::<RichTextLabel>("LogBox");
        self.log_box = Some(log_box);

        // Get Button and connect signal
        let mut button = self.base().get_node_as::<TextureButton>("RunButton");

        button.connect("pressed", &self.base_mut().callable("on_run_pressed"));

        //let tilemap = self.base().get_node_as::<TileMapLayer>("logic_map");
    }

    fn process(&mut self, delta: f32) {
        if let Some(logic_map) = &self.logic_map {
            if let Some(character) = &mut self.scripted_character {
                let mut char_bind = character.bind_mut();
                if let Some(logic) = &mut char_bind.logic {
                    let result = self.executor.tick(delta, logic, logic_map);

                    if result != ExecutorResult::Empty {
                        // highlight the current executing line in the editor
                        let current_line = self.executor.current_line();

                        if let Some(code_editor) = &mut self.code_editor {
                            code_editor.set_caret_line(current_line as i32 - 1);
                            code_editor.center_viewport_to_caret();
                        }
                    }
                }
            }
        }

        let input = Input::singleton();

        if input.is_action_just_pressed("toggle_debug_overlay") {
            let mut overlay = self.base().get_node_as::<DebugOverlay>("DebugOverlay");
            overlay.bind_mut().toggle();
        }

        if input.is_action_just_pressed("toggle_grid_overlay") {
            let mut overlay = self.base().get_node_as::<GridOverlay>("GridOverlay");
            overlay.bind_mut().toggle();
        }

        //process the script and ScriptedCharacter
        // editor.set_caret_line(line_number as i32 - 1);
        // editor.center_viewport_to_caret();
    }
}
