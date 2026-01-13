use crate::logger::{LogType, Logger};
use std::sync::OnceLock;

static LOGGGER: OnceLock<Box<dyn Logger>> = OnceLock::new();

pub fn init_logger(logger: Box<dyn Logger>) {
    let _ = LOGGGER.set(logger);
}

#[inline]
pub fn log(level: LogType, msg: &str) {
    if let Some(l) = LOGGGER.get() {
        l.log(level, msg);
    }
}
