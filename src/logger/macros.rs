
#[macro_export]
/// ## Description
/// Logs a verbose log message in the log file.
///
/// ### Parameters
/// A reference to TheLogger's instance and the log message itself
///
/// ### Example
/// ```rust
/// use the_logger::{log, TheLogger};
///
/// async fn exec_log() {
///   let logger = TheLogger::instance();
///   log!(logger, "This is a verbose log message");
/// }
/// ```
macro_rules! log {
    ($logger:expr, $($msg:tt)*) => {
        $logger.verbose().await.log_in_file((file!(), line!(), column!()), &format!($($msg)*)).await;
    };
}

/// ## Description
/// Logs an informational log message in the log file.
///
/// ### Parameters
/// A reference to TheLogger's instance and the log message itself
///
/// ### Example
/// ```rust
/// use the_logger::{log_info, TheLogger};
///
/// async fn exec_log() {
///   let logger = TheLogger::instance();
///   log_info!(logger, "This is an informational log message");
/// }
/// ```
#[macro_export]
macro_rules! log_info {
    ($logger:expr, $($msg:tt)*) => {

        $logger.info().await.log_in_file((file!(), line!(), column!()), &format!($($msg)*)).await;
    };
}

/// ## Description
/// Logs an error log message in the log file.
///
/// ### Parameters
/// A reference to TheLogger's instance and the log message itself
///
/// ### Example
/// ```rust
/// use the_logger::{log_error, TheLogger};
///
/// async fn exec_log() {
///   let logger = TheLogger::instance();
///   log_error!(logger, "This is an error log message");
/// }
/// ```
#[macro_export]
macro_rules! log_error {
    ($logger:expr, $($msg:tt)*) => {
        $logger.error().await.log_in_file((file!(), line!(), column!()), &format!($($msg)*)).await;
    };
}

/// ## Description
/// Logs a warning log message in the log file.
///
/// ### Parameters
/// A reference to TheLogger's instance and the log message itself
///
/// ### Example
/// ```rust
/// use the_logger::{log_warning, TheLogger};
///
/// async fn exec_log() {
///   let logger = TheLogger::instance();
///   log_warning!(logger, "This is a warning log message");
/// }
/// ```
#[macro_export]
macro_rules! log_warning {
    ($logger:expr, $($msg:tt)*) => {

        $logger.warning().await.log_in_file((file!(), line!(), column!()), &format!($($msg)*)).await;
    };
}

/// ## Description
/// Logs a debug log message in the log file.
///
/// ### Parameters
/// A reference to TheLogger's instance and the log message itself
///
/// ### Example
/// ```rust
/// use the_logger::{log_debug, TheLogger};
///
/// async fn exec_log() {
///   let logger = TheLogger::instance();
///   log_debug!(logger, "This is a debug log message");
/// }
/// ```
#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $($msg:tt)*) => {

        $logger.debug().await.log_in_file((file!(), line!(), column!()), &format!($($msg)*)).await;
    };
}

/// ## Description
/// Logs a trace log message in the log file.
///
/// ### Parameters
/// A reference to TheLogger's instance and the log message itself
///
/// ### Example
/// ```rust
/// use the_logger::{log_trace, TheLogger};
///
/// async fn exec_log() {
///   let logger = TheLogger::instance();
///   log_trace!(logger, "This is a trace log message");
/// }
/// ```
#[macro_export]
macro_rules! log_trace {
    ($logger:expr, $($msg:tt)*) => {

        $logger.trace().await.log_in_file((file!(), line!(), column!()), &format!($($msg)*)).await;
    };
}

/// ## Description
/// Logs a critical log message in the log file.
///
/// ### Parameters
/// A reference to TheLogger's instance and the log message itself
///
/// ### Example
/// ```rust
/// use the_logger::{log_critical, TheLogger};
///
/// async fn exec_log() {
///   let logger = TheLogger::instance();
///   log_critical!(logger, "This is a critical log message");
/// }
/// ```
#[macro_export]
macro_rules! log_critical {
    ($logger:expr, $($msg:tt)*) => {

        $logger.critical().await.log_in_file((file!(), line!(), column!()), &format!($($msg)*)).await;
    };
}
