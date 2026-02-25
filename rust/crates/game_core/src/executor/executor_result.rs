#[derive(Debug, PartialEq)]
pub enum ExecutorResult {
    Running,
    NotIdle,
    Empty,
    Turn,
}
