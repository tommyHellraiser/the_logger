
#[macro_export]
macro_rules! log {
    ($logger:expr, $($msg:tt)*) => {
        $logger.verbose().await.log_in_file((file!(), line!(), column!()), &format!($($msg)*)).await;
    };
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $($msg:tt)*) => {

        $logger.info().await.log_in_file((file!(), line!(), column!()), &format!($($msg)*)).await;
    };
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $($msg:tt)*) => {
        $logger.error().await.log_in_file((file!(), line!(), column!()), &format!($($msg)*)).await;
    };
}

#[macro_export]
macro_rules! log_warn {
    ($logger:expr, $($msg:tt)*) => {

        $logger.warning().await.log_in_file((file!(), line!(), column!()), &format!($($msg)*)).await;
    };
}

#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $($msg:tt)*) => {

        $logger.debug().await.log_in_file((file!(), line!(), column!()), &format!($($msg)*)).await;
    };
}

#[macro_export]
macro_rules! log_trace {
    ($logger:expr, $($msg:tt)*) => {

        $logger.trace().await.log_in_file((file!(), line!(), column!()), &format!($($msg)*)).await;
    };
}

#[macro_export]
macro_rules! log_critical {
    ($logger:expr, $($msg:tt)*) => {

        $logger.critical().await.log_in_file((file!(), line!(), column!()), &format!($($msg)*)).await;
    };
}
