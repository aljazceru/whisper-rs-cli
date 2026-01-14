use owo_colors::OwoColorize;
use std::sync::atomic::{AtomicBool, Ordering};

static SILENT: AtomicBool = AtomicBool::new(false);

pub fn set_silent(silent: bool) {
    SILENT.store(silent, Ordering::SeqCst);
}

pub fn is_silent() -> bool {
    SILENT.load(Ordering::SeqCst)
}

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

pub fn log(level: LogLevel, message: &str) {
    if is_silent() {
        return;
    }

    match level {
        LogLevel::Info => eprintln!("{}", message.bold()),
        LogLevel::Success => eprintln!("{}", message.green()),
        LogLevel::Warning => eprintln!("{}", message.yellow()),
        LogLevel::Error => eprintln!("{}", message.red().bold()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_silent() {
        set_silent(true);
        assert!(is_silent());

        set_silent(false);
        assert!(!is_silent());
    }

    #[test]
    fn test_is_silent_default() {
        let default_state = is_silent();
        set_silent(default_state);
    }

    #[test]
    fn test_set_silent_multiple_times() {
        set_silent(true);
        assert!(is_silent());

        set_silent(true);
        assert!(is_silent());

        set_silent(false);
        assert!(!is_silent());

        set_silent(false);
        assert!(!is_silent());
    }

    #[test]
    fn test_log_levels() {
        let levels = vec![
            LogLevel::Info,
            LogLevel::Success,
            LogLevel::Warning,
            LogLevel::Error,
        ];
        assert_eq!(levels.len(), 4);
    }

    #[test]
    fn test_log_level_debug() {
        let info = LogLevel::Info;
        let success = LogLevel::Success;
        let warning = LogLevel::Warning;
        let error = LogLevel::Error;

        let _ = format!("{:?}", info);
        let _ = format!("{:?}", success);
        let _ = format!("{:?}", warning);
        let _ = format!("{:?}", error);
    }

    #[test]
    fn test_log_silent_mode() {
        set_silent(true);
        log(LogLevel::Info, "Test message");
        set_silent(false);
    }

    #[test]
    fn test_log_macros_compilation() {
        set_silent(true);
        crate::log_info!("Test info {}", "message");
        crate::log_success!("Test success {}", "message");
        crate::log_warning!("Test warning {}", "message");
        crate::log_error!("Test error {}", "message");
        set_silent(false);
    }
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::output::logger::log($crate::output::logger::LogLevel::Info, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_success {
    ($($arg:tt)*) => {
        $crate::output::logger::log($crate::output::logger::LogLevel::Success, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        $crate::output::logger::log($crate::output::logger::LogLevel::Warning, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::output::logger::log($crate::output::logger::LogLevel::Error, &format!($($arg)*));
    };
}
