//Logging interface
#[derive(Debug, PartialEq)]
pub enum LogType {
    warn,
    info,
    error,
    debug,
    trace,
}

pub trait Logger: Send + Sync {
    fn log(&self, log_type: LogType, msg: &str);
}
