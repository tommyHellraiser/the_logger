
#[derive(Debug)]
pub struct TheLoggerConfig {
    date_config: TheDateConfig,
    time_config: TheTimeConfig,
    misc_config: TheMiscConfig,
    log_level: LogLevel
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
    hide_level: bool,
    hide_file_name: bool,
    hide_file_line: bool,
    show_file_column: bool,
    location_length: usize,
    log_content_length: usize
}

#[derive(Default, Debug, Copy, Clone)]
pub(super) enum LogLevel {
    #[default]
    Verbose,
    Information,
    Error,
    Warning,
    Debug,
    Trace,
    Critical
}

impl TheLoggerConfig {
    // Log Type methods
    /// ## Description
    /// Configures the log level as verbose adding the [[VERBOSE]] tag
    pub async fn verbose(mut self) -> Self {
        self.log_level = LogLevel::Verbose;
        self
    }

    /// ## Description
    /// Configures the log level as informational adding the [[INFO]] tag
    pub async fn info(mut self) -> Self {
        self.log_level = LogLevel::Information;
        self
    }

    /// ## Description
    /// Configures the log level as error adding the [[ERROR]] tag
    pub async fn error(mut self) -> Self{
        self.log_level = LogLevel::Error;
        self
    }

    /// ## Description
    /// Configures the log level as warning adding the [[WARNING]] tag
    pub async fn warning(mut self) -> Self {
        self.log_level = LogLevel::Warning;
        self
    }

    /// ## Description
    /// Configures the log level as debug adding the [[DEBUG]] tag
    pub async fn debug(mut self) -> Self {
        self.log_level = LogLevel::Debug;
        self
    }

    /// ## Description
    /// Configures the log level as trace adding the [[TRACE]] tag
    pub async fn trace(mut self) -> Self {
        self.log_level = LogLevel::Trace;
        self
    }

    /// ## Description
    /// Configures the log level as critical adding the [[CRITICAL]] tag
    pub async fn critical(mut self) -> Self {
        self.log_level = LogLevel::Critical;
        self
    }

    // Configuration methods
    /// ## Description
    /// Configures the log date to hide the years. Default is to show them
    pub fn hide_years(mut self) -> Self {
        self.date_config.hide_years = true;
        self
    }

    /// ## Description
    /// Configures the log date to hide the months. Default is to show them
    pub fn hide_months(mut self) -> Self {
        self.date_config.hide_months = true;
        self
    }

    /// ## Description
    /// Configures the log date to hide the days. Default is to show them
    pub fn hide_days(mut self) -> Self {
        self.date_config.hide_days = true;
        self
    }

    /// ## Description
    /// Configures the log time to hide the hours. Default is to show them
    pub fn hide_hours(mut self) -> Self {
        self.time_config.hide_hours = true;
        self
    }

    /// ## Description
    /// Configures the log time to hide the minutes. Default is to show them
    pub fn hide_minutes(mut self) -> Self {
        self.time_config.hide_minutes = true;
        self
    }

    /// ## Description
    /// Configures the log time to hide the seconds. Default is to show them
    pub fn hide_seconds(mut self) -> Self {
        self.time_config.hide_seconds = true;
        self
    }

    /// ## Description
    /// Configures the log time to hide the milliseconds. Default is to show them
    ///
    /// ### Warning
    /// Hiding the milliseconds and showing the microseconds would cause an unexpected time tracking in the logs,
    ///
    /// therefore, in this specific case, both milliseconds and microseconds will be hidden
    pub fn hide_millisecs(mut self) -> Self {
        self.time_config.hide_millisecs = true;
        self
    }

    /// ## Description
    /// Configures the log time to hide the microseconds. Default is to show them
    ///
    /// ### Warning
    /// Hiding the milliseconds and showing the microseconds would cause an unexpected time tracking in the logs,
    ///
    /// therefore, in this specific case, both milliseconds and microseconds will be hidden
    pub fn hide_microsecs(mut self) -> Self {
        self.time_config.hide_microsecs = true;
        self
    }

    /// ## Description
    /// Configures the log timezone to UTC format. Default is Local time
    pub fn utc_time(mut self) -> Self {
        self.time_config.utc_time = true;
        self
    }

    /// ## Description
    /// Configures the log level to be hidden. Default is to show it
    pub fn hide_level(mut self) -> Self {
        self.misc_config.hide_level = true;
        self
    }

    /// ## Description
    /// Configures the log file location to be hidden. Default is to show it
    pub fn hide_file_name(mut self) -> Self {
        self.misc_config.hide_file_name = true;
        self
    }

    /// ## Description
    /// Configures the log file line number to be hidden. Default is to show it
    pub fn hide_file_line(mut self) -> Self {
        self.misc_config.hide_file_line = true;
        self
    }

    /// ## Description
    /// Configures the log file column number to be hidden. Default is to hide it
    pub fn hide_file_column(mut self) -> Self {
        self.misc_config.show_file_column = false;
        self
    }

    /// ## Description
    /// Configures the log date to show the years. Default is to show them
    pub fn show_years(mut self) -> Self {
        self.date_config.hide_years = false;
        self
    }

    /// ## Description
    /// Configures the log date to show the months. Default is to show them
    pub fn show_months(mut self) -> Self {
        self.date_config.hide_months = false;
        self
    }

    /// ## Description
    /// Configures the log date to show the days. Default is to show them
    pub fn show_days(mut self) -> Self {
        self.date_config.hide_days = false;
        self
    }

    /// ## Description
    /// Configures the log time to show the hours. Default is to show them
    pub fn show_hours(mut self) -> Self {
        self.time_config.hide_hours = false;
        self
    }

    /// ## Description
    /// Configures the log time to show the minutes. Default is to show them
    pub fn show_minutes(mut self) -> Self {
        self.time_config.hide_minutes = false;
        self
    }

    /// ## Description
    /// Configures the log time to show the seconds. Default is to show them
    pub fn show_seconds(mut self) -> Self {
        self.time_config.hide_seconds = false;
        self
    }

    /// ## Description
    /// Configures the log time to show the milliseconds. Default is to show them
    ///
    /// ### Warning
    /// Hiding the milliseconds and showing the microseconds would cause an unexpected time tracking in the logs,
    ///
    /// therefore, in this specific case, both milliseconds and microseconds will be hidden
    pub fn show_millisecs(mut self) -> Self {
        self.time_config.hide_millisecs = false;
        self
    }

    /// ## Description
    /// Configures the log time to show the microseconds. Default is to show them
    ///
    /// ### Warning
    /// Hiding the milliseconds and showing the microseconds would cause an unexpected time tracking in the logs,
    ///
    /// therefore, in this specific case, both milliseconds and microseconds will be hidden
    pub fn show_microsecs(mut self) -> Self {
        self.time_config.hide_microsecs = false;
        self
    }

    /// ## Description
    /// Configures the log timezone to Local format. Default is Local time
    pub fn local_time(mut self) -> Self {
        self.time_config.utc_time = false;
        self
    }

    /// ## Description
    /// Configures the log level to be shown. Default is to show it
    pub fn show_level(mut self) -> Self {
        self.misc_config.hide_level = false;
        self
    }

    /// ## Description
    /// Configures the log file location to be shown. Default is to show it
    pub fn show_file_name(mut self) -> Self {
        self.misc_config.hide_file_name = false;
        self
    }

    /// ## Description
    /// Configures the log file line number to be shown. Default is to show it
    pub fn show_file_line(mut self) -> Self {
        self.misc_config.hide_file_line = false;
        self
    }

    /// ## Description
    /// Configures the log file column number to be shown. Default is to hide it
    pub fn show_file_column(mut self) -> Self {
        self.misc_config.show_file_column = true;
        self
    }

    /// ## Description
    /// Configures the log file name, line and column location content's length. Default is 100 characters
    pub fn location_content_length(mut self, length: usize) -> Self {
        self.misc_config.location_length = length;
        self
    }

    /// ## Description
    /// Configures the log message content's length. Default is 300 characters
    pub fn log_content_length(mut self, length: usize) -> Self {
        self.misc_config.log_content_length = length;
        self
    }

    //  Getters
    #[doc(hidden)]
    pub(super) fn get_years_config(&self) -> bool {
        self.date_config.hide_years
    }

    #[doc(hidden)]
    pub(super) fn get_months_config(&self) -> bool {
        self.date_config.hide_months
    }

    #[doc(hidden)]
    pub(super) fn get_days_config(&self) -> bool {
        self.date_config.hide_days
    }

    #[doc(hidden)]
    pub(super) fn get_hours_config(&self) -> bool {
        self.time_config.hide_hours
    }

    #[doc(hidden)]
    pub(super) fn get_minutes_config(&self) -> bool {
        self.time_config.hide_minutes
    }

    #[doc(hidden)]
    pub(super) fn get_seconds_config(&self) -> bool {
        self.time_config.hide_seconds
    }

    #[doc(hidden)]
    pub(super) fn get_millisecs_config(&self) -> bool {
        self.time_config.hide_millisecs
    }

    #[doc(hidden)]
    pub(super) fn get_microsecs_config(&self) -> bool {
        self.time_config.hide_microsecs
    }

    #[doc(hidden)]
    pub(super) fn get_utc_config(&self) -> bool {
        self.time_config.utc_time
    }

    #[doc(hidden)]
    pub(super) fn get_level_config(&self) -> bool {
        self.misc_config.hide_level
    }

    #[doc(hidden)]
    pub(super) fn get_file_name_config(&self) -> bool {
        self.misc_config.hide_file_name
    }

    #[doc(hidden)]
    pub(super) fn get_file_line_config(&self) -> bool {
        self.misc_config.hide_file_line
    }

    #[doc(hidden)]
    pub(super) fn get_file_column_config(&self) -> bool {
        self.misc_config.show_file_column
    }

    #[doc(hidden)]
    pub(super) fn get_log_level(&self) -> LogLevel {
        self.log_level
    }

    #[doc(hidden)]
    pub(super) fn get_location_length(&self) -> usize {
        self.misc_config.location_length
    }

    #[doc(hidden)]
    pub(super) fn get_log_content_length(&self) -> usize {
        self.misc_config.log_content_length
    }

    //  Setters
    #[doc(hidden)]
    pub(super) fn set_years_config(&mut self, data: bool) {
        self.date_config.hide_years = data;
    }

    #[doc(hidden)]
    pub(super) fn set_months_config(&mut self, data: bool) {
        self.date_config.hide_months = data;
    }

    #[doc(hidden)]
    pub(super) fn set_days_config(&mut self, data: bool) {
        self.date_config.hide_days = data;
    }

    #[doc(hidden)]
    pub(super) fn set_hours_config(&mut self, data: bool) {
        self.time_config.hide_hours = data;
    }

    #[doc(hidden)]
    pub(super) fn set_minutes_config(&mut self, data: bool) {
        self.time_config.hide_minutes = data;
    }

    #[doc(hidden)]
    pub(super) fn set_seconds_config(&mut self, data: bool) {
        self.time_config.hide_seconds = data;
    }

    #[doc(hidden)]
    pub(super) fn set_millisecs_config(&mut self, data: bool) {
        self.time_config.hide_millisecs = data;
    }

    #[doc(hidden)]
    pub(super) fn set_microsecs_config(&mut self, data: bool) {
        self.time_config.hide_microsecs = data;
    }

    #[doc(hidden)]
    pub(super) fn set_utc_config(&mut self, data: bool) {
        self.time_config.utc_time = data;
    }

    #[doc(hidden)]
    pub(super) fn set_level_config(&mut self, data: bool) {
        self.misc_config.hide_level = data;
    }

    // #[doc(hidden)]
    // pub(super) fn set_file_name_config(&mut self, data: bool) {
    //     self.misc_config.hide_file_name = data;
    // }
    //
    // #[doc(hidden)]
    // pub(super) fn set_file_line_config(&mut self, data: bool) {
    //     self.misc_config.hide_file_line = data;
    // }
    //
    // #[doc(hidden)]
    // pub(super) fn set_file_column_config(&mut self, data: bool) {
    //     self.misc_config.show_file_column = data;
    // }

    #[doc(hidden)]
    pub(super) fn set_log_level(&mut self, data: LogLevel) {
        self.log_level = data;
    }

    #[doc(hidden)]
    pub(super) fn set_location_length(&mut self, data: usize) {
        self.misc_config.location_length = data;
    }

    #[doc(hidden)]
    pub(super) fn set_log_content_length(&mut self, data: usize) {
        self.misc_config.log_content_length = data;
    }
}

impl Default for TheLoggerConfig {
    fn default() -> Self {
        Self {
            date_config: TheDateConfig::default(),
            time_config: TheTimeConfig::default(),
            misc_config: TheMiscConfig {
                location_length: 80,
                log_content_length: 300,
                ..Default::default()
            },
            log_level: LogLevel::Verbose
        }
    }
}
