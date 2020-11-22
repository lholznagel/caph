use log::{Level, Log, Metadata, Record, SetLoggerError};
use std::time::Instant;

/// The logger instance
/// To configure the logger, use the `LogBuilder` struct
pub struct Morgan {
    pub(crate) time: Instant,
    pub(crate) color: bool,
}

impl Morgan {
    /// Creates a new logger using the default values
    ///
    /// # Defaults
    /// - `level` -> The default level is `Info`
    /// - `exclude` -> No targets are excluded
    ///
    /// # Example
    /// ```
    /// #[macro_use]
    /// extern crate log;
    /// extern crate morgan;
    ///
    /// use morgan::Morgan;
    ///
    /// fn main() {
    ///     Morgan::init().unwrap();
    ///
    ///     error!("My error message");
    ///     warn!("My warn message");
    ///     info!("My info message");
    ///     debug!("Will not be shown");
    ///     trace!("Will not be shown");
    /// }
    /// ```
    pub fn init() -> Result<(), SetLoggerError> {
        let morgan = Self::default();
        log::set_boxed_logger(Box::new(morgan))?;

        let log_level = std::env::var("MORGAN_LEVEL")
            .map(|x| match x.to_lowercase().as_ref() {
                "debug" => Level::Debug,
                "error" => Level::Error,
                _ => Level::Info,
            })
            .unwrap_or(Level::Info);
        log::set_max_level(log_level.to_level_filter());
        Ok(())
    }
}

impl Log for Morgan {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let exclude = ["surf", "tracing", "isahc", "hyper", "tide", "sqlx"];
        let base = metadata.target().split(':').next().unwrap_or_default();
        !exclude.contains(&base)
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let lvl_msg = if self.color {
            match record.level() {
                Level::Error => "\x1B[0;31mError \x1B",
                Level::Warn => "\x1B[0;93mWarn  \x1B",
                Level::Info => "\x1B[0;34mInfo  \x1B",
                Level::Debug => "\x1B[0;35mDebug \x1B",
                Level::Trace => "\x1B[0;36mTrace \x1B",
            }
        } else {
            match record.level() {
                Level::Error => "Error ",
                Level::Warn => "Warn  ",
                Level::Info => "Info  ",
                Level::Debug => "Debug ",
                Level::Trace => "Trace ",
            }
        };

        if self.color {
            println!(
                "\x1B[1;30m[{:10.3?}] > \x1B {}[1;30m>\x1B[0m {}",
                self.time.elapsed().as_secs_f64(),
                lvl_msg,
                record.args()
            );
        } else {
            println!(
                "[{:10.3?}] > {} > {}",
                self.time.elapsed().as_secs_f64(),
                lvl_msg,
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

impl Default for Morgan {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            color: std::env::var("MORGAN_COLOR")
                .map(|x| x.parse().unwrap_or(true))
                .unwrap_or(true),
        }
    }
}
