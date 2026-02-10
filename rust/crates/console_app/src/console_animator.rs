use std::collections::HashMap;

use platform::shared::logger_global::log;

use platform::types::Vector2D;
use platform::{animator::Animator, logger::LogType};

pub struct ConsoleAnimator {
    frames: HashMap<&'static str, (usize, bool)>, // animation name and (amount of frames, loop)
    current_animation: String,
    current_frame: usize,
    position: Vector2D,
}

impl ConsoleAnimator {
    pub fn new() -> Self {
        log(LogType::Debug, "Creating ConsoleAnimator");
        let mut frames = HashMap::new();
        frames.insert("stand_south", (1, true));
        frames.insert("stand_north", (1, true));
        frames.insert("stand_west", (1, true));
        frames.insert("stand_east", (1, true));
        frames.insert("turn_south_east", (3, false));
        frames.insert("turn_east_south", (3, false));
        frames.insert("turn_south_west", (3, false));
        frames.insert("turn_west_north", (3, false));
        frames.insert("turn_north_east", (3, false));

        Self {
            frames,
            current_animation: "".to_string(),
            current_frame: 0,
            position: Vector2D::new(0.0, 0.0),
        }
    }
}

impl Animator for ConsoleAnimator {
    fn play(&mut self, name: &str) {
        log(LogType::Info, &format!("Start playing animation: {}", name));
        self.current_animation = name.to_string();
        self.current_frame = 0;
    }

    fn is_playing(&self) -> bool {
        log(LogType::Debug, "is_playing called");
        if let Some(anim) = self.frames.get(self.current_animation.as_str()) {
            if anim.1 {
                log(LogType::Debug, "\ntrue");
                return true;
            }
            if self.current_frame >= anim.0 {
                log(LogType::Debug, "\nfalse");
                return false;
            } else {
                log(LogType::Debug, "\ntrue");
                return true;
            }
        }
        log(LogType::Debug, "\nfalse");
        false
    }

    fn process(&mut self, _delta: f32) {
        if let Some(anim) = self.frames.get(self.current_animation.as_str()) {
            self.current_frame += 1;
            if anim.1 {
                if self.current_frame >= anim.0 {
                    self.current_frame = 0;
                }
            } else {
                if self.current_frame >= anim.0 {
                    self.current_frame = anim.0;
                }
            }
        }
    }

    fn set_position(&mut self, position: Vector2D) {
        self.position = position;
    }

    fn get_position(&self) -> Vector2D {
        self.position
    }

    fn get_global_position(&self) -> Vector2D {
        self.position
    }
}
