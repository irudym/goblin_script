use crate::logger::{LogType, Logger};
use std::sync::OnceLock;

static LOGGER: OnceLock<Box<dyn Logger>> = OnceLock::new();

pub fn init_logger(logger: Box<dyn Logger>) {
    let _ = LOGGER.set(logger);
}

#[inline]
pub fn log(level: LogType, msg: &str) {
    if let Some(l) = LOGGER.get() {
        l.log(level, msg);
    }
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {{
        log($level, &format!($($arg)*));
    }
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Error, $($arg)*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Warn, $($arg)*);
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Info, $($arg)*);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::log!($crate::LogLevel::Debug, $($arg)*);
    };
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::log!($crate::LogLevel::Trace, $($arg)*);
    };
}
