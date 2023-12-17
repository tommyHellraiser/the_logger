use std::fs;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::Write;
use tokio::sync::RwLock;
use crate::logger::logger_config::{LogLevel, TheLoggerConfig};

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



impl TheLogger {
    fn new() -> Self {
        fs::create_dir_all("./logs/").unwrap();
        Self {
            inner: RwLock::new(TheLoggerInner {
                config: TheLoggerConfig::default(),
                file_writer: std::fs::OpenOptions::new()
                                     .write(true)
                                     .create(true)
                                     .append(true)
                                     .open(
                                         format!("./logs/Log {}.log", chrono::Local::now().naive_local().format("%Y-%m-%d"))
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
    /// - File name and line number shown, but column number is hidden.
    /// - File name, location, column information section is limited to a maximum default of 60 chars, configurable.
    /// - Log content is limited to a maximum default of 300 chars, also configurable.
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
    pub async fn log_in_file(&self, (file, line, column): (&str, u32, u32), incoming_msg: &str) {
        let mut msg = String::new();
        let mut location_info = String::new();
        let location_length;
        let content_length;

        {
            let inner = self.inner.read().await;

            //  Datetime formatting
            let datetime = if inner.config.get_utc_config() {
                chrono::Utc::now().naive_utc()
            } else {
                chrono::Local::now().naive_local()
            };
            let mut datetime_format = String::new();

            //  Date formatting
            let mut space_date_time = false;
            if !inner.config.get_years_config() {
                datetime_format.push_str("%Y");
                space_date_time = true;
            }
            if !inner.config.get_months_config() {
                space_date_time = true;
                datetime_format.push_str("-%m");
            }
            if !inner.config.get_days_config() {
                space_date_time = true;
                datetime_format.push_str("-%d");
            }
            if space_date_time {
                datetime_format.push(' ');
            }

            //  Time formatting
            if !inner.config.get_hours_config() {
                datetime_format.push_str("%H");
            }
            if !inner.config.get_minutes_config() {
                datetime_format.push_str(":%M");
            }
            if !inner.config.get_seconds_config() {
                datetime_format.push_str(":%S");
            }
            match (inner.config.get_millisecs_config(), inner.config.get_microsecs_config()) {
                (false, false) => datetime_format.push_str(".%6f"),
                (false, true) => datetime_format.push_str(".%3f"),
                (true, false) => {},
                _ => {}
            }
            datetime_format.push('\t');
            msg.push_str(&datetime.format(&datetime_format).to_string());

            //  Log level type config
            if !inner.config.get_level_config() {
                match inner.config.get_log_level() {
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
            } else {
                msg.push('\t');
            }

            //  File location configuration
            if !inner.config.get_file_name_config() {
                location_info.push_str(format!("@{}", file).as_str());

                if !inner.config.get_file_line_config() {
                    location_info.push_str(format!(": {}", line).as_str());

                    if inner.config.get_file_column_config() {
                        location_info.push_str(format!("|{}", column).as_str());
                    }
                }
            }

            //  Location info and content lengths configuration
            location_length = inner.config.get_location_length();
            content_length = inner.config.get_log_content_length();
        }

        //  Close the message string and log it. Trim the message if it's longer than the configured lengths
        if location_length < location_info.len() {
            location_info = location_info.as_str()[0..location_length].to_string();
            //  Append a tab to separate the location info and the content a little bit
            location_info.push_str("\t\t");
        }
        if !location_info.is_empty() {
            msg.push_str(format!("{:<location_length$}", location_info).as_str());
        }
        if content_length < incoming_msg.len() {
            msg.push_str(incoming_msg[0..content_length].as_ref());
        } else {
            msg.push_str(incoming_msg);
        }
        self.inner.write().await.file_writer.write_all(format!("{:<content_length$}\n", msg).as_bytes()).unwrap();
    }

    /// ## Description
    /// Allows the user to configure the logger using a single config call. The configuration must previously be created
    /// by instantiating the TheLoggerConfig struct adn setting each desired field with its builder methods.
    ///
    /// Every change to the configuration is made through this method.
    ///
    /// ### Initial configuration example
    /// ```rust
    /// use the_logger::logger::{logger_config::TheLoggerConfig, the_logger::TheLogger};
    ///
    /// async fn init_logger() {
    ///     let logger_config = TheLoggerConfig::default()
    ///         .hide_file_line()
    ///         .hide_microsecs()
    ///         .hide_level();
    ///
    ///     let logger: &TheLogger = TheLogger::instance().config(logger_config).await;
    /// }
    /// ```
    ///
    /// As a result, the user has configured the logger to hide the file line number, the microsecs and the log level,
    ///
    /// and leave the rest of the parameters with their default values.
    ///
    /// ### Modifying configuration example
    /// If the user needs to modify the logger configuration for any particular reason, for example for a specific
    ///
    /// routine that doesn't need to show all the data or needs to show different data, this can be achieved in a similar
    ///
    /// way to the initial configuration:
    ///
    /// ```rust
    /// use the_logger::logger::{logger_config::TheLoggerConfig, the_logger::TheLogger};
    ///
    /// async fn config_logger() {
    ///     let logger_config = TheLoggerConfig::default()
    ///         .show_file_line()
    ///         .show_microsecs()
    ///         .show_level();
    ///
    ///     let logger: &TheLogger = TheLogger::instance().config(logger_config).await;
    /// }
    /// ```
    ///
    /// As opposed to the previous example, we're now reverting the changes made to the logger configuration, so that
    ///
    /// the file line number, microseconds stamp and log level are shown.
    pub async fn config(&self, logger_config: TheLoggerConfig) -> &Self {
        self.inner.write().await.config = logger_config;
        self
    }

    // Log Type methods
    /// ## Description
    /// Configures the log level as verbose adding the [[VERBOSE]] tag
    pub async fn verbose(&self) -> &Self {
        self.inner.write().await.config.set_log_level(LogLevel::Verbose);
        self
    }

    /// ## Description
    /// Configures the log level as informational adding the [[INFO]] tag
    pub async fn info(&self) -> &Self {
        self.inner.write().await.config.set_log_level(LogLevel::Information);
        self
    }

    /// ## Description
    /// Configures the log level as error adding the [[ERROR]] tag
    pub async fn error(&self) -> &Self{
        self.inner.write().await.config.set_log_level(LogLevel::Error);
        self
    }

    /// ## Description
    /// Configures the log level as warning adding the [[WARNING]] tag
    pub async fn warning(&self) -> &Self {
        self.inner.write().await.config.set_log_level(LogLevel::Warning);
        self
    }

    /// ## Description
    /// Configures the log level as debug adding the [[DEBUG]] tag
    pub async fn debug(&self) -> &Self {
        self.inner.write().await.config.set_log_level(LogLevel::Debug);
        self
    }

    /// ## Description
    /// Configures the log level as trace adding the [[TRACE]] tag
    pub async fn trace(&self) -> &Self {
        self.inner.write().await.config.set_log_level(LogLevel::Trace);
        self
    }

    /// ## Description
    /// Configures the log level as critical adding the [[CRITICAL]] tag
    pub async fn critical(&self) -> &Self {
        self.inner.write().await.config.set_log_level(LogLevel::Critical);
        self
    }

    // Configuration methods
    /// ## Description
    /// Configures the log date to hide the years. Default is to show them
    pub async fn hide_years(&self) -> &Self {
        self.inner.write().await.config.set_years_config(true);
        self
    }

    /// ## Description
    /// Configures the log date to hide the months. Default is to show them
    pub async fn hide_months(&self) -> &Self {
        self.inner.write().await.config.set_months_config(true);
        self
    }

    /// ## Description
    /// Configures the log date to hide the days. Default is to show them
    pub async fn hide_days(&self) -> &Self {
        self.inner.write().await.config.set_days_config(true);
        self
    }

    /// ## Description
    /// Configures the log time to hide the hours. Default is to show them
    pub async fn hide_hours(&self) -> &Self {
        self.inner.write().await.config.set_hours_config(true);
        self
    }

    /// ## Description
    /// Configures the log time to hide the minutes. Default is to show them
    pub async fn hide_minutes(&self) -> &Self {
        self.inner.write().await.config.set_minutes_config(true);
        self
    }

    /// ## Description
    /// Configures the log time to hide the seconds. Default is to show them
    pub async fn hide_seconds(&self) -> &Self {
        self.inner.write().await.config.set_seconds_config(true);
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
        self.inner.write().await.config.set_millisecs_config(true);
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
        self.inner.write().await.config.set_microsecs_config(true);
        self
    }

    /// ## Description
    /// Configures the log timezone to UTC format. Default is Local time
    pub async fn utc_time(&self) -> &Self {
        self.inner.write().await.config.set_utc_config(true);
        self
    }

    /// ## Description
    /// Configures the log level to be hidden. Default is to show it
    pub async fn hide_level(&self) -> &Self {
        self.inner.write().await.config.set_level_config(true);
        self
    }

    /// ## Description
    /// Configures the log date to show the years. Default is to show them
    pub async fn show_years(&self) -> &Self {
        self.inner.write().await.config.set_years_config(false);
        self
    }

    /// ## Description
    /// Configures the log date to show the months. Default is to show them
    pub async fn show_months(&self) -> &Self {
        self.inner.write().await.config.set_months_config(false);
        self
    }

    /// ## Description
    /// Configures the log date to show the days. Default is to show them
    pub async fn show_days(&self) -> &Self {
        self.inner.write().await.config.set_days_config(false);
        self
    }

    /// ## Description
    /// Configures the log time to show the hours. Default is to show them
    pub async fn show_hours(&self) -> &Self {
        self.inner.write().await.config.set_hours_config(false);
        self
    }

    /// ## Description
    /// Configures the log time to show the minutes. Default is to show them
    pub async fn show_minutes(&self) -> &Self {
        self.inner.write().await.config.set_minutes_config(false);
        self
    }

    /// ## Description
    /// Configures the log time to show the seconds. Default is to show them
    pub async fn show_seconds(&self) -> &Self {
        self.inner.write().await.config.set_seconds_config(false);
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
        self.inner.write().await.config.set_millisecs_config(false);
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
        self.inner.write().await.config.set_microsecs_config(false);
        self
    }

    /// ## Description
    /// Configures the log timezone to Local format. Default is Local time
    pub async fn local_time(&self) -> &Self {
        self.inner.write().await.config.set_utc_config(false);
        self
    }

    /// ## Description
    /// Configures the log level to be shown. Default is to show it
    pub async fn show_level(&self) -> &Self {
        self.inner.write().await.config.set_level_config(false);
        self
    }

    /// ## Description
    /// Configures the log file name, line and column location content's length. Default is 100 characters
    pub async fn location_content_length(&self, length: usize) -> &Self {
        self.inner.write().await.config.set_location_length(length);
        self
    }

    /// ## Description
    /// Configures the log message content's length. Default is 300 characters
    pub async fn log_content_length(&self, length: usize) -> &Self {
        self.inner.write().await.config.set_log_content_length(length);
        self
    }
}
