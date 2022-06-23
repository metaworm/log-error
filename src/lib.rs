#![doc = include_str!("../README.md")]

use core::fmt::{Debug, Display};
use log::{logger, Level};

/// Trait to log the error result, there are impls for [`Result`] and [`Option`] by default.
pub trait LogError<T, E>: Sized {
    /// log the error with specific log-level and format handler
    fn log_level_with<F: FnOnce(E) -> String>(self, level: Level, cb: F) -> Option<T>;

    /// log a error message with specific format handler
    #[inline(always)]
    fn log_error_with<F: FnOnce(E) -> String>(self, cb: F) -> Option<T> {
        self.log_level_with(Level::Error, cb)
    }

    /// log a warn message with specific format handler
    #[inline(always)]
    fn log_warn_with<F: FnOnce(E) -> String>(self, cb: F) -> Option<T> {
        self.log_level_with(Level::Warn, cb)
    }

    /// log the error with specific prefix
    #[inline(always)]
    fn log_error(self, msg: &str) -> Option<T>
    where
        E: Display,
    {
        self.log_error_with(|err| format!("{msg}: {err}"))
    }

    /// log the error with specific prefix in a detailed format
    #[inline(always)]
    fn log_error_detail(self, msg: &str) -> Option<T>
    where
        E: Debug,
    {
        self.log_error_with(|err| format!("{msg}: {err:#?}"))
    }

    /// log the error with specific prefix as a warn message
    #[inline(always)]
    fn log_warn(self, msg: &str) -> Option<T>
    where
        E: Display,
    {
        self.log_warn_with(|err| format!("{msg}: {err}"))
    }

    /// log the error with specific prefix in a detailed format as a warn message
    #[inline(always)]
    fn log_warn_detail(self, msg: &str) -> Option<T>
    where
        E: Debug,
    {
        self.log_warn_with(|err| format!("{msg}: {err:#?}"))
    }
}

/// Implements [`LogError`] for [`Result`]
impl<T, E> LogError<T, E> for Result<T, E> {
    #[inline(always)]
    fn log_level_with<F: FnOnce(E) -> String>(self, level: Level, cb: F) -> Option<T> {
        match self {
            Ok(res) => Some(res),
            Err(err) => {
                log_message(level, cb(err));
                None
            }
        }
    }
}

/// Implements [`LogError`] for [`Option`]
impl<T> LogError<T, &'static str> for Option<T> {
    #[inline(always)]
    fn log_level_with<F: FnOnce(&'static str) -> String>(self, level: Level, cb: F) -> Option<T> {
        match self {
            Some(res) => Some(res),
            None => {
                log_message(level, cb("None"));
                None
            }
        }
    }
}

fn log_message(level: Level, msg: String) {
    // TODO: use backtrace to get line number
    logger().log(
        &log::Record::builder()
            .args(format_args!("{msg}"))
            .level(level)
            .module_path_static("log-error".into())
            .build(),
    );
}
