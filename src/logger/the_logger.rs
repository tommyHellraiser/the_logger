
use lazy_static::lazy_static;
use std::fs::File;
use std::io::Write;
use tokio::sync::RwLock;

lazy_static!(
    static ref THE_LOGGER: TheLogger = TheLogger::new();
);

pub struct TheLogger {
    inner: RwLock<TheLoggerInner>
}

struct TheLoggerInner {
    config: TheLoggerConfig,
    file_writer: File
}

#[derive(Default, Debug)]
struct TheLoggerConfig {
    date_config: TheDateConfig,
    time_config: TheTimeConfig,
    misc_config: TheMiscConfig,
    log_level: LogLevel
}

#[derive(Default, Debug)]
pub enum LogLevel {
    #[default]
    Verbose,
    Information,
    Error,
    Warning,
    Debug,
    Trace,
    Critical
}

#[derive(Default, Debug)]
struct TheDateConfig {
    hide_years: bool,
    hide_months: bool,
    hide_days: bool,
}

#[derive(Default, Debug)]
struct TheTimeConfig {
    hide_hours: bool,
    hide_minutes: bool,
    hide_seconds: bool,
    hide_millisecs: bool,
    hide_microsecs: bool,
    utc_time: bool
}

#[derive(Default, Debug)]
struct TheMiscConfig {
    hide_level: bool
}

impl TheLogger {
    fn new() -> Self {
        Self {
            inner: RwLock::new(TheLoggerInner {
                config: TheLoggerConfig::default(),
                file_writer: std::fs::OpenOptions::new()
                                     .write(true)
                                     .create(true)
                                     .append(true)
                                     .open(
                                         format!("Log {}.log", chrono::Local::now().naive_local().format("%Y-%m-%d"))
                                     ).unwrap()
            })
        }
    }

    /// ## Description
    /// Returns the instance of the logger. This is the main way to access the logger, and it'll be configured by default.
    ///
    /// Its setup can be changed later using the methods provided by this crate.
    ///
    /// ### Default settings:
    /// - All date elements enabled
    /// - All time elements enabled
    /// - Log level enabled and set to VERBOSE
    /// - Time format set to Local time
    ///
    /// ### Log Example
    /// ```text
    /// 2023-12-16 17:08:07.451851800  [VERBOSE]  This is a log example for the the_logger crate
    /// ```
    pub fn instance() -> &'static Self {
        &THE_LOGGER
    }

    /// ## Description
    /// Executes the logging to the file according to the current configuration
    pub async fn log_in_file(&self, incoming_msg: &str) {
        let mut msg = String::new();

        {
            let inner = self.inner.read().await;

            //  Datetime formatting
            let datetime = if inner.config.time_config.utc_time {
                chrono::Utc::now().naive_utc()
            } else {
                chrono::Local::now().naive_local()
            };
            let mut datetime_format = String::new();

            //  Date formatting
            let mut space_date_time = false;
            if !inner.config.date_config.hide_years {
                datetime_format.push_str("%Y");
                space_date_time = true;
            }
            if !inner.config.date_config.hide_months {
                space_date_time = true;
                datetime_format.push_str("-%m");
            }
            if !inner.config.date_config.hide_days {
                space_date_time = true;
                datetime_format.push_str("-%d");
            }
            if space_date_time {
                datetime_format.push(' ');
            }

            //  Time formatting
            if !inner.config.time_config.hide_hours {
                datetime_format.push_str("%H");
            }
            if !inner.config.time_config.hide_minutes {
                datetime_format.push_str(":%M");
            }
            if !inner.config.time_config.hide_seconds {
                datetime_format.push_str(":%S");
            }
            match (inner.config.time_config.hide_millisecs, inner.config.time_config.hide_microsecs) {
                (false, false) => datetime_format.push_str(".%6f"),
                (false, true) => datetime_format.push_str(".%3f"),
                (true, false) => {},
                _ => {}
            }
            datetime_format.push('\t');
            msg.push_str(&datetime.format(&datetime_format).to_string());

            //  Log level type config
            if !inner.config.misc_config.hide_level {
                match inner.config.log_level {
                    LogLevel::Verbose => {
                        msg.push_str("[VERBOSE]\t");
                    },
                    LogLevel::Information => {
                        msg.push_str("[INFO]\t\t");
                    },
                    LogLevel::Error => {
                        msg.push_str("[ERROR]\t\t");
                    },
                    LogLevel::Warning => {
                        msg.push_str("[WARNING]\t");
                    },
                    LogLevel::Debug => {
                        msg.push_str("[DEBUG]\t\t");
                    },
                    LogLevel::Trace => {
                        msg.push_str("[TRACE]\t\t");
                    },
                    LogLevel::Critical => {
                        msg.push_str("[CRITICAL]\t");
                    }
                }
            }
        }

        msg.push_str(incoming_msg);
        self.inner.write().await.file_writer.write_all(msg.as_bytes()).unwrap();
    }

    // Log Type methods
    /// ## Description
    /// Configures the log level as verbose adding the [[VERBOSE]] tag
    pub async fn verbose(&self) -> &Self {
        self.inner.write().await.config.log_level = LogLevel::Verbose;
        self
    }

    /// ## Description
    /// Configures the log level as informational adding the [[INFO]] tag
    pub async fn info(&self) -> &Self {
        self.inner.write().await.config.log_level = LogLevel::Information;
        self
    }

    /// ## Description
    /// Configures the log level as error adding the [[ERROR]] tag
    pub async fn error(&self) -> &Self{
        self.inner.write().await.config.log_level = LogLevel::Error;
        self
    }

    /// ## Description
    /// Configures the log level as warning adding the [[WARNING]] tag
    pub async fn warning(&self) -> &Self {
        self.inner.write().await.config.log_level = LogLevel::Warning;
        self
    }

    /// ## Description
    /// Configures the log level as debug adding the [[DEBUG]] tag
    pub async fn debug(&self) -> &Self {
        self.inner.write().await.config.log_level = LogLevel::Debug;
        self
    }

    /// ## Description
    /// Configures the log level as trace adding the [[TRACE]] tag
    pub async fn trace(&self) -> &Self {
        self.inner.write().await.config.log_level = LogLevel::Trace;
        self
    }

    /// ## Description
    /// Configures the log level as critical adding the [[CRITICAL]] tag
    pub async fn critical(&self) -> &Self {
        self.inner.write().await.config.log_level = LogLevel::Critical;
        self
    }

    // Configuration methods
    /// ## Description
    /// Configures the log date to hide the years. Default is to show them
    pub async fn hide_years(&self) -> &Self {
        self.inner.write().await.config.date_config.hide_years = true;
        self
    }

    /// ## Description
    /// Configures the log date to hide the months. Default is to show them
    pub async fn hide_months(&self) -> &Self {
        self.inner.write().await.config.date_config.hide_months = true;
        self
    }

    /// ## Description
    /// Configures the log date to hide the days. Default is to show them
    pub async fn hide_days(&self) -> &Self {
        self.inner.write().await.config.date_config.hide_days = true;
        self
    }

    /// ## Description
    /// Configures the log time to hide the hours. Default is to show them
    pub async fn hide_hours(&self) -> &Self {
        self.inner.write().await.config.time_config.hide_hours = true;
        self
    }

    /// ## Description
    /// Configures the log time to hide the minutes. Default is to show them
    pub async fn hide_minutes(&self) -> &Self {
        self.inner.write().await.config.time_config.hide_minutes = true;
        self
    }

    /// ## Description
    /// Configures the log time to hide the seconds. Default is to show them
    pub async fn hide_seconds(&self) -> &Self {
        self.inner.write().await.config.time_config.hide_seconds = true;
        self
    }

    /// ## Description
    /// Configures the log time to hide the milliseconds. Default is to show them
    ///
    /// ### Warning
    /// Hiding the milliseconds and showing the microseconds would cause an unexpected time tracking in the logs,
    ///
    /// therefore, in this specific case, both milliseconds and microseconds will be hidden
    pub async fn hide_millisecs(&self) -> &Self {
        self.inner.write().await.config.time_config.hide_millisecs = true;
        self
    }

    /// ## Description
    /// Configures the log time to hide the microseconds. Default is to show them
    ///
    /// ### Warning
    /// Hiding the milliseconds and showing the microseconds would cause an unexpected time tracking in the logs,
    ///
    /// therefore, in this specific case, both milliseconds and microseconds will be hidden
    pub async fn hide_microsecs(&self) -> &Self {
        self.inner.write().await.config.time_config.hide_microsecs = true;
        self
    }

    /// ## Description
    /// Configures the log timezone to UTC format. Default is Local time
    pub async fn utc_time(&self) -> &Self {
        self.inner.write().await.config.time_config.utc_time = true;
        self
    }

    /// ## Description
    /// Configures the log level to be hidden. Default is to show it
    pub async fn hide_level(&self) -> &Self {
        self.inner.write().await.config.misc_config.hide_level = true;
        self
    }

    /// ## Description
    /// Configures the log date to show the years. Default is to show them
    pub async fn show_years(&self) -> &Self {
        self.inner.write().await.config.date_config.hide_years = false;
        self
    }

    /// ## Description
    /// Configures the log date to show the months. Default is to show them
    pub async fn show_months(&self) -> &Self {
        self.inner.write().await.config.date_config.hide_months = false;
        self
    }

    /// ## Description
    /// Configures the log date to show the days. Default is to show them
    pub async fn show_days(&self) -> &Self {
        self.inner.write().await.config.date_config.hide_days = false;
        self
    }

    /// ## Description
    /// Configures the log time to show the hours. Default is to show them
    pub async fn show_hours(&self) -> &Self {
        self.inner.write().await.config.time_config.hide_hours = false;
        self
    }

    /// ## Description
    /// Configures the log time to show the minutes. Default is to show them
    pub async fn show_minutes(&self) -> &Self {
        self.inner.write().await.config.time_config.hide_minutes = false;
        self
    }

    /// ## Description
    /// Configures the log time to show the seconds. Default is to show them
    pub async fn show_seconds(&self) -> &Self {
        self.inner.write().await.config.time_config.hide_seconds = false;
        self
    }

    /// ## Description
    /// Configures the log time to show the milliseconds. Default is to show them
    ///
    /// ### Warning
    /// Hiding the milliseconds and showing the microseconds would cause an unexpected time tracking in the logs,
    ///
    /// therefore, in this specific case, both milliseconds and microseconds will be hidden
    pub async fn show_millisecs(&self) -> &Self {
        self.inner.write().await.config.time_config.hide_millisecs = false;
        self
    }

    /// ## Description
    /// Configures the log time to show the microseconds. Default is to show them
    ///
    /// ### Warning
    /// Hiding the milliseconds and showing the microseconds would cause an unexpected time tracking in the logs,
    ///
    /// therefore, in this specific case, both milliseconds and microseconds will be hidden
    pub async fn show_microsecs(&self) -> &Self {
        self.inner.write().await.config.time_config.hide_microsecs = false;
        self
    }

    /// ## Description
    /// Configures the log timezone to Local format. Default is Local time
    pub async fn local_time(&self) -> &Self {
        self.inner.write().await.config.time_config.utc_time = false;
        self
    }

    /// ## Description
    /// Configures the log level to be shown. Default is to show it
    pub async fn show_level(&self) -> &Self {
        self.inner.write().await.config.misc_config.hide_level = false;
        self
    }
}
