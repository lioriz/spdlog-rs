//! Provides sinks to flexibly output log messages to specified targets.

pub mod file_sink;
pub mod rotating_file_sink;
pub mod std_out_stream_sink;
pub mod std_out_stream_style_sink;

pub use file_sink::FileSink;
pub use rotating_file_sink::{RotatingFileSink, RotationPolicy};
pub use std_out_stream_sink::StdOutStreamSink;
pub use std_out_stream_style_sink::StdOutStreamStyleSink;

use std::sync::Arc;

use crate::{formatter::Formatter, Level, LevelFilter, Record, Result};

/// A trait for sinks.
///
/// Sinks are the objects that actually write the log to their target. Each sink
/// should be responsible for only single target (e.g file, console, db), and
/// each sink has its own private instance of [`Formatter`] object.
///
/// A [`Logger`] can combine multiple [`Sink`] s.
///
/// [`Logger`]: crate::logger::Logger
pub trait Sink: Sync + Send {
    /// Determines if a log message with the specified level would be logged.
    fn should_log(&self, level: Level) -> bool {
        self.level_filter().compare(level)
    }

    /// Logs the record.
    ///
    /// Internally filtering the log message level is redundant, it should be
    /// filtered by the caller by calling [`Sink::should_log`]. Its
    /// implementation should guarantee that it will never panic even if the
    /// caller did not filter it by calling [`Sink::should_log`], otherwise it
    /// should always filter these potential panic cases internally.
    fn log(&self, record: &Record) -> Result<()>;

    /// Flushes any buffered records.
    fn flush(&self) -> Result<()>;

    /// Getter of the log level filter.
    fn level_filter(&self) -> LevelFilter;

    /// Setter of the log level filter.
    fn set_level_filter(&self, level_filter: LevelFilter);

    /// Swaps the formatter.
    fn swap_formatter(&self, formatter: Box<dyn Formatter>) -> Box<dyn Formatter>;

    /// Setter of the formatter.
    fn set_formatter(&self, formatter: Box<dyn Formatter>) {
        self.swap_formatter(formatter);
    }
}

/// A container for [`Sink`]s.
pub type Sinks = Vec<Arc<dyn Sink>>;
