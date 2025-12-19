use godot::classes::{AnimatedSprite2D, Area2D, IArea2D, Input};
use godot::prelude::*;

use std::sync::{Arc, Mutex};

use crate::bt::blackboard::{Blackboard, BlackboardValue};
use crate::bt::leafs::{IsAtTarget, MoveToTarget, NextWaypoint, Wait};
use crate::bt::nodes::{Selector, Sequence};
use crate::bt::BoxBTNode;
use crate::fsm::run::RunState;
use crate::fsm::turn::TurnState;
use crate::fsm::walk::WalkState;
use crate::fsm::StateType;
use crate::fsm::{idle::IdleState, Direction, FSM};

use crate::character::request::StateRequest;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Character {
    pub direction: Direction,
    pub speed: f32,
    state: Option<Box<dyn FSM>>, // the active state machine, accessible only by Main Thread
    brain: Option<BoxBTNode>,    // character AI

    blackboard: Blackboard,
    pending_request: Arc<Mutex<Option<StateRequest>>>, // the request buffer, thread safe

    base: Base<Area2D>,
}

impl Character {
    pub fn snap_to_cell(&mut self) {
        // get cell i,j
        let position = self.base().get_position();
        let i = f32::round(position.x / 32.0) as i32;
        let j = f32::round(position.y / 32.0) as i32;

        self.base_mut().set_position(Vector2 {
            x: (i * 32) as f32,
            y: (j * 32) as f32,
        });
    }

    // check if animation is still in process, keep out the switching to new animation
    pub fn is_playing(&self) -> bool {
        let sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        sprite.is_playing()
    }

    pub fn play_animation(&mut self, animation_name: &str) {
        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        sprite.set_animation(animation_name);
        sprite.play();
    }

    // check if the character is in the idle state
    pub fn is_idle(&self) -> bool {
        self.state.as_ref().unwrap().get_type() == StateType::IDLE
    }
}

#[godot_api]
impl IArea2D for Character {
    fn init(base: Base<Area2D>) -> Self {
        let route = vec![
            Vector2::new(0.0, 0.0),
            Vector2::new(5.0, 0.0), // Move 5 tiles East
            Vector2::new(5.0, 5.0), // Move 5 tiles South
            Vector2::new(0.0, 5.0), // Return home
        ];

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

        let blackboard = Blackboard::new();
        let first_point = Vector2 {
            x: route[0].x * 32.0,
            y: route[0].y * 32.0,
        };
        blackboard.set("target_pos", BlackboardValue::Vector(first_point));

        let root = Box::new(Selector::new(vec![
            Box::new(Sequence::new(vec![
                Box::new(IsAtTarget::new("target_pos")),
                Box::new(Wait::new(0.8)),
                Box::new(NextWaypoint::new(route, "target_pos")),
            ])),
            Box::new(MoveToTarget::new("target_pos")),
        ]));

        Character {
            state: Some(Box::new(IdleState::new())),
            brain: Some(root),
            blackboard: blackboard,
            direction: Direction::SOUTH,
            pending_request: Arc::new(Mutex::new(None)),
            speed: 50.0,
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("Character {} loaded", self.base().get_name());
        //self.snap_to_cell();
        self.base_mut().set_position(Vector2 { x: 0.0, y: 0.0 });

        if let Some(mut state) = self.state.take() {
            state.enter(self);
            self.state = Some(state);
        }
    }

    fn process(&mut self, delta: f32) {
        if let Some(mut brain) = self.brain.take() {
            let bb = self.blackboard.clone();

            brain.tick(self, &bb, delta);
            self.brain = Some(brain);
        }

        // Process pending request
        self.handle_transitions();

        // Update the current state
        if let Some(mut state) = self.state.take() {
            state.update(delta, self);
            self.state = Some(state);
        }

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
    }
}
