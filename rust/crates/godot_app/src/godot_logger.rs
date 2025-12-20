use godot::prelude::*;
use platform::logger::{LogType, Logger};

pub struct GodotLogger {}

impl GodotLogger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Logger for GodotLogger {
    fn log(&self, log_type: LogType, msg: &str) {
        use LogType::*;
        let prefix = match log_type {
            debug => "[D]",
            info => "[I]",
            warn => "[W]",
            error => "[E]",
            trace => "[T]",
        };
        godot_print!("{}: {}", prefix, msg);
    }
}
