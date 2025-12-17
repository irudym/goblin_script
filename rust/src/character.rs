use godot::classes::{AnimatedSprite2D, Area2D, IArea2D, Input};
use godot::prelude::*;

use crate::fsm::run::RunState;
use crate::fsm::turn::TurnState;
use crate::fsm::{Direction, FSM, idle::IdleState};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Character {
    pub direction: Direction,
    state: Option<Box<dyn FSM>>,
    base: Base<Area2D>,
}

impl Character {
    fn snap_to_cell(&mut self) {
        // get cell i,j
        let position = self.base().get_position();
        let i = f32::floor(position.x / 32.0) as i32;
        let j = f32::floor(position.y / 32.0).floor() as i32;

        self.base_mut().set_position(Vector2 {
            x: (i * 32) as f32,
            y: (j * 32) as f32,
        });
    }

    /*
     * Set and play animation by construction animation name using Character direction:
     * for example, anumation is run, the direction is WEST, than animation name will be run_west
     */
    pub fn play_animation_with_direction(&mut self, animation_name: &str) {
        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let animation = format!("{}_{}", animation_name, self.direction);
        sprite.set_animation(&animation);
        sprite.play();
    }

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

    fn change_state(&mut self, mut new_state: Box<dyn FSM>) {
        //check if locked
        if let Some(state) = self.state.take() {
            godot_print!(
                "State type: {:?}, can exit: {:?}",
                state.get_type(),
                state.can_exit()
            );
            if state.can_exit() {
                if state.can_transition_to(new_state.get_type()) {
                    state.exit(self);
                    new_state.enter(self);
                    self.state = Some(new_state);
                    return;
                }
            }
            self.state = Some(state);
        }
    }
}

#[godot_api]
impl IArea2D for Character {
    fn init(base: Base<Area2D>) -> Self {
        Character {
            state: Some(Box::new(IdleState::new())),
            direction: Direction::NORTH,
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("Character {} loaded", self.base().get_name());
        self.snap_to_cell();

        if let Some(mut state) = self.state.take() {
            state.enter(self);
            self.state = Some(state);
        }
        //let anim_names = sprite.get_sprite_frames().unwrap().get_animation_names();
        //godot_print!("Animations: {:?}", anim_names);
    }

    fn process(&mut self, delta: f32) {
        if let Some(mut state) = self.state.take() {
            state.update(delta, self);
            self.state = Some(state);
        }

        let input = Input::singleton();
        //godot_print!("Key: {}", input.is_anything_pressed());
        if input.is_action_pressed("move_right") {
            if self.direction != Direction::EAST {
                self.change_state(Box::new(IdleState::new()));
                self.change_state(Box::new(TurnState::new(Direction::EAST)));
            } else {
                self.change_state(Box::new(RunState::new()));
            }
        }
        if input.is_action_pressed("move_left") {
            if self.direction != Direction::WEST {
                self.change_state(Box::new(IdleState::new()));
                self.change_state(Box::new(TurnState::new(Direction::WEST)));
            } else {
                self.change_state(Box::new(RunState::new()));
            }
        }
        if input.is_action_pressed("move_down") {
            if self.direction != Direction::SOUTH {
                self.change_state(Box::new(IdleState::new()));
                self.change_state(Box::new(TurnState::new(Direction::SOUTH)));
            } else {
                self.change_state(Box::new(RunState::new()));
            }
        }
        if input.is_action_just_released("move_right")
            || input.is_action_just_released("move_left")
            || input.is_action_just_released("move_down")
        {
            self.change_state(Box::new(IdleState::new()));
        }
    }
}
