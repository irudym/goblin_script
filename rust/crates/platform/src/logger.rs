//Logging interface
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogType {
    Warn,
    Info,
    Error,
    Debug,
    Trace,
}

pub trait Logger: Send + Sync {
    fn log(&self, log_type: LogType, msg: &str);
}
