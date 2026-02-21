use game_core::character::CharacterId;
use game_core::map::LogicMap;
use godot::classes::{AnimatedSprite2D, Area2D, IArea2D};
use godot::prelude::*;
use platform::logger::LogType;

use crate::godot_animator::GodotAnimator;
use game_core::bt::leafs::{IsAtTarget, MoveToTarget, NextWaypoint, Wait};
use game_core::bt::nodes::{Selector, Sequence};
use game_core::bt::{BTRef, BehaviourTree};
use game_core::CharacterLogic;
use platform::types::Vector2D;

//use platform::shared::logger_global::log;

use platform::{log_error, log_info};
use std::sync::Arc;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Character {
    base: Base<Area2D>,
    logic: Option<CharacterLogic>,
    logic_map: Option<Arc<LogicMap>>,
    tile_size: f32,
}

//#[godot_api]
impl Character {
    //#[func]
    pub fn set_logic_map(&mut self, logic_map: Arc<LogicMap>) {
        self.logic_map = Some(logic_map);
    }

    // check if animation is still in process, keep out the switching to new animation
    fn build_tree(&self) -> BTRef {
        // test patrol
        // Tree structure:
        // Sequence
        //  1. Patrol Logic (update target, return Success when arrived)
        //  2. Wait (pause)
        //  3. WalkToTarget

        // Actually, a better structure for continuous movement is:
        // Selector
        //   1. Sequence (Target Reached?)
        //       a. Patrol (Calculate next point if arrived)
        //       b. Wait (Visualize looking around)
        //   2. WalkToTarget (Keep moving to current target)
        let route = if let Some(points) = self.get_patrol_point() {
            points
        } else {
            vec![
                Vector2D::new(0.0, 0.0),
                Vector2D::new(5.0, 0.0), // Move 5 tiles East
                Vector2D::new(5.0, 5.0), // Move 5 tiles South
                Vector2D::new(0.0, 5.0), // Return home
            ]
        };

        log_info!("Patrol points: {:?}", route);

        Arc::new(BehaviourTree::new(Box::new(Selector::new(vec![
            Box::new(Sequence::new(vec![
                Box::new(NextWaypoint::new(route, "target_pos", self.tile_size)),
                Box::new(Wait::new(0.64)),
                Box::new(IsAtTarget::new("target_pos")),
            ])),
            Box::new(MoveToTarget::new("target_pos")),
        ]))))
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

    fn get_patrol_point(&self) -> Option<Vec<Vector2D>> {
        let mut result = Vec::new();

        let variant = self.base().get_meta("patrol");

        if variant.is_nil() {
            return None;
        }

        let array = match variant.try_to::<Array<Variant>>() {
            Ok(arr) => arr,
            Err(_) => {
                log_error!("Meta 'patrol' is not an Array<Vector2i>");
                return None;
            }
        };

        for v in array.iter_shared() {
            if let Ok(vec2i) = v.try_to::<Vector2i>() {
                result.push(Vector2D {
                    x: vec2i.x as f32,
                    y: vec2i.y as f32,
                });
            } else {
                log_error!("Meta 'patrol' contains non Vector2i value");
            }
        }

        Some(result)
    }
}

#[godot_api]
impl IArea2D for Character {
    fn init(base: Base<Area2D>) -> Self {
        Character {
            base,
            logic: None,
            logic_map: None,
            tile_size: 64.0,
        }
    }

    fn ready(&mut self) {
        let name = self.base().get_name();
        log_info!("Character {} loaded", &name);

        let sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        let animator = Box::new(GodotAnimator::new(sprite));

        //build BT tree
        let tree = self.build_tree();

        //get id from meta
        let id = self.get_id();
        log_info!("Character[{}] id: {}", &name, id);

        let mut logic = CharacterLogic::new(id, animator);

        logic.bt = tree;
        if let Some(points) = self.get_patrol_point() {
            logic.set_cell_position(points[0].x as i32, points[0].y as i32);
        }

        self.logic = Some(logic);

        // let scene = (self.base().get_parent()).get_parent().unwrap();
        // let scene = scene.cast::<Scene>();
    }

    fn process(&mut self, delta: f32) {
        if let Some(logic) = &mut self.logic {
            if let Some(logic_map) = &mut self.logic_map {
                logic.process(delta, &logic_map);
            }
        }

        /*
        // Handle input
        let input = Input::singleton();
        //godot_print!("Key: {}", input.is_anything_pressed());
        if input.is_action_pressed("move_right") {
            if self.direction != Direction::EAST {
                self.request_state(StateRequest::Turn(Direction::EAST));
            } else {
                self.request_state(StateRequest::Run);
            }
        }
        if input.is_action_pressed("move_left") {
            if self.direction != Direction::WEST {
                self.request_state(StateRequest::Turn(Direction::WEST));
            } else {
                self.request_state(StateRequest::Run);
            }
        }
        if input.is_action_pressed("move_down") {
            if self.direction != Direction::SOUTH {
                self.request_state(StateRequest::Turn(Direction::SOUTH));
            } else {
                self.request_state(StateRequest::Run);
            }
        }
        if input.is_action_just_released("move_right")
            || input.is_action_just_released("move_left")
            || input.is_action_just_released("move_down")
        {
            self.request_state(StateRequest::Idle);
        }
        */
    }
}
