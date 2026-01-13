use log::{debug, error, info, trace, warn};
use platform::logger::{LogType, Logger};

use colored::Colorize;

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, log_type: LogType, msg: &str) {
        use LogType::*;
        match log_type {
            Warn => warn!("{}", msg.yellow()),
            Info => info!("{}", msg.green()),
            Error => error!("{}", msg.red()),
            Debug => debug!("{}", msg),
            Trace => trace!("{}", msg.purple()),
        }
    }
}
