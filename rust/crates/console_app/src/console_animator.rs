use std::collections::HashMap;

use crate::console_logger::ConsoleLogger;

use platform::{
    animator::Animator,
    logger::{LogType, Logger},
};

pub struct ConsoleAnimator {
    logger: ConsoleLogger,
    frames: HashMap<&'static str, (usize, bool)>, // animation name and (amount of frames, loop)
    current_animation: String,
    current_frame: usize,
}

impl ConsoleAnimator {
    pub fn new() -> Self {
        let logger = ConsoleLogger::new();
        logger.log(LogType::debug, "Creating ConsoleAnimator");
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
            logger,
            frames,
            current_animation: "".to_string(),
            current_frame: 0,
        }
    }
}

impl Animator for ConsoleAnimator {
    fn play(&mut self, name: &str) {
        self.logger
            .log(LogType::info, &format!("Start playing animation: {}", name));
        self.current_animation = name.to_string();
        self.current_frame = 0;
    }

    fn is_playing(&self) -> bool {
        self.logger.log(LogType::debug, "is_playing called");
        if let Some(anim) = self.frames.get(self.current_animation.as_str()) {
            if anim.1 {
                self.logger.log(LogType::debug, "\ntrue");
                return true;
            }
            if self.current_frame >= anim.0 {
                self.logger.log(LogType::debug, "\nfalse");
                return false;
            } else {
                self.logger.log(LogType::debug, "\ntrue");
                return true;
            }
        }
        self.logger.log(LogType::debug, "\nfalse");
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
}
