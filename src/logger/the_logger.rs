#![allow(dead_code)]
use lazy_static::lazy_static;
use tokio::sync::RwLock;

//  TODO: determine if it's possible to receive either a string message or a TheError struct inside a macro
#[derive(Default)]
#[allow(dead_code)]
pub enum LogLevel {
    Fatal,
    Error,
    Warning,
    #[default]
    Information,
    Debug,
    Trace
}

lazy_static!(
    static ref THE_LOGGER: TheLogger = TheLogger::new();
);

pub struct TheLogger {
    config: RwLock<TheLoggerConfig>
}

#[derive(Default, Debug)]
struct DateData {
    year: u16,
    month: Month,
    day: u8
}

#[derive(Default, Debug)]
enum Month {
    #[default]
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December
}



struct TimeData {

}

#[derive(Default, Debug)]
pub struct TheLoggerConfig {
    date_config: TheDateConfig,
    time_config: TheTimeConfig,
    format_config: TheFormatConfig,
    misc_config: TheMiscConfig
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
    hide_millisecs: bool
}

#[derive(Default, Debug)]
struct TheFormatConfig {
    //  TODO: implement this, pending to interpret format from string
    //date_format: String
    //  TODO: not implemented yet, pending to interpret format from string
    //timestamp_format: String,
}

#[derive(Default, Debug)]
struct TheMiscConfig {
    hide_level: bool,
    level_enclosure: TheEnclosureType,
    hide_error_line: bool,
    hide_error_column: bool
}

#[derive(Default, Debug)]
pub enum TheEnclosureType {
    #[default]
    Square,
    Curly,
    Parenthesis
}



impl TheLogger {
    pub fn new() -> Self {
        Self {
            config: RwLock::new(TheLoggerConfig::default())
        }
    }
}

impl TheLoggerConfig {
    pub fn new() -> Self {
        Self {
            date_config: TheDateConfig::default(),
            time_config: TheTimeConfig::default(),
            format_config: TheFormatConfig::default(),
            misc_config: TheMiscConfig::default()
        }
    }

    pub fn hide_date(mut self) -> Self {
        self.date_config.hide_years = true;
        self.date_config.hide_months = true;
        self.date_config.hide_days = true;
        self
    }

    pub fn hide_years(mut self) -> Self{
        self.date_config.hide_years = true;
        self
    }

    pub fn hide_months(mut self) -> Self {
        self.date_config.hide_months = true;
        self
    }

    pub fn hide_days(mut self) -> Self {
        self.date_config.hide_days = true;
        self
    }

    pub fn hide_timestamp(mut self) -> Self {
        self.time_config.hide_hours = true;
        self.time_config.hide_minutes = true;
        self.time_config.hide_seconds = true;
        self.time_config.hide_millisecs = true;
        self
    }

    pub fn timestamp_format(mut self) -> Self {
        todo!()
    }

    pub fn hide_hours(mut self) -> Self {
        self.time_config.hide_hours = true;
        self
    }

    pub fn hide_minutes(mut self) -> Self {
        self.time_config.hide_minutes = true;
        self
    }

    pub fn hide_secs(mut self) -> Self {
        self.time_config.hide_seconds = true;
        self
    }

    pub fn hide_millisecs(mut self) -> Self {
        self.time_config.hide_millisecs = true;
        self
    }

    pub fn hide_level(mut self) -> Self {
        self.misc_config.hide_level = true;
        self
    }

    pub fn level_enclosure(mut self, level_config: TheEnclosureType) -> Self {
        self.misc_config.level_enclosure = level_config;
        self
    }

    pub fn hide_error_line(mut self) -> Self {
        self.misc_config.hide_error_line = true;
        self
    }

    pub fn hide_error_column(mut self) -> Self {
        self.misc_config.hide_error_column = true;
        self
    }
}



/*
YYYY-MM-DD HH:MM:SS.mm LEVEL \t @ FILE LINE:COL -> Error itself
 */