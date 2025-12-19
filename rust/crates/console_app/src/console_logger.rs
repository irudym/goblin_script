use log::{debug, error, info, trace, warn};
use platform::logger::{LogType, Logger};

use colored::Colorize;

pub struct ConsoleLogger {}

impl ConsoleLogger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Logger for ConsoleLogger {
    fn log(&self, log_type: LogType, msg: &str) {
        use LogType::*;
        match log_type {
            warn => warn!("{}", msg.yellow()),
            info => info!("{}", msg.green()),
            error => error!("{}", msg.red()),
            debug => debug!("{}", msg),
            trace => trace!("{}", msg.purple()),
        }
    }
}
