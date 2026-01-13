use godot::prelude::*;
use platform::logger::{LogType, Logger};

pub struct GodotLogger;

impl Logger for GodotLogger {
    fn log(&self, log_type: LogType, msg: &str) {
        use LogType::*;
        let prefix = match log_type {
            Debug => "[D]",
            Info => "[I]",
            Warn => "[W]",
            Error => "[E]",
            Trace => "[T]",
        };
        godot_print!("{}: {}", prefix, msg);
    }
}
