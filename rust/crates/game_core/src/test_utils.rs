/// Shared test initialization utilities.
/// Provides a single global `ensure_init()` so that `init_bt_system()`
/// and `init_logger()` are called exactly once regardless of how many
/// test modules use them.
#[cfg(test)]
pub mod test_init {
    use crate::ai::worker::init_bt_system;
    use platform::logger::{LogType, Logger};
    use platform::shared::logger_global::init_logger;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn ensure_init() {
        INIT.call_once(|| {
            struct NullLogger;
            impl Logger for NullLogger {
                fn log(&self, _: LogType, _: &str) {}
            }
            init_logger(Box::new(NullLogger));
            init_bt_system();
        });
    }
}
