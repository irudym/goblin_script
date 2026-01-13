use godot::classes::{AnimatedSprite2D, Area2D, IArea2D};
use godot::prelude::*;

use crate::godot_animator::GodotAnimator;
use crate::godot_logger::GodotLogger;
use game_core::ai::worker::init_bt_system;
use game_core::bt::leafs::{IsAtTarget, MoveToTarget, NextWaypoint, Wait};
use game_core::bt::nodes::{Selector, Sequence};
use game_core::bt::{BTRef, BehaviourTree};
use game_core::CharacterLogic;
use platform::types::Vector2D;

use std::sync::Arc;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Character {
    base: Base<Area2D>,
    logic: Option<CharacterLogic>,
}

impl Character {
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
        let route = vec![
            Vector2D::new(0.0, 0.0),
            Vector2D::new(5.0, 0.0), // Move 5 tiles East
            Vector2D::new(5.0, 5.0), // Move 5 tiles South
            Vector2D::new(0.0, 5.0), // Return home
        ];

        Arc::new(BehaviourTree::new(Box::new(Selector::new(vec![
            Box::new(Sequence::new(vec![
                Box::new(NextWaypoint::new(route, "target_pos")),
                Box::new(Wait::new(0.32)),
                Box::new(IsAtTarget::new("target_pos")),
            ])),
            Box::new(MoveToTarget::new("target_pos")),
        ]))))
    }
}

#[godot_api]
impl IArea2D for Character {
    fn init(base: Base<Area2D>) -> Self {
        Character { base, logic: None }
    }

    fn ready(&mut self) {
        godot_print!("Character {} loaded", self.base().get_name());
        //self.snap_to_cell();
        self.base_mut().set_position(Vector2 { x: 0.0, y: 0.0 });

        let sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        let animator = Box::new(GodotAnimator::new(sprite));
        let logger = Box::new(GodotLogger::new());

        //build BT tree
        let tree = self.build_tree();

        init_bt_system();

        let mut logic = CharacterLogic::new(1, animator, logger);
        logic.bt = tree;
        self.logic = Some(logic);
    }

    fn process(&mut self, delta: f32) {
        if let Some(logic) = &mut self.logic {
            logic.process(delta);
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
