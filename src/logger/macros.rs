
#[macro_export]
macro_rules! log {
    ($logger:expr, $($msg:tt)*) => {
        let message = format!($($msg)*);
        $logger.verbose().await.log_in_file(&message).await;
    };
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $($msg:tt)*) => {

        let message = format!($($msg)*);
        $logger.info().await.log_in_file(&message).await;
    };
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $($msg:tt)*) => {

        let message = format!($($msg)*);
        $logger.error().await.log_in_file(&message).await;
    };
}

#[macro_export]
macro_rules! log_warn {
    ($logger:expr, $($msg:tt)*) => {

        let message = format!($($msg)*);
        $logger.warning().await.log_in_file(&message).await;
    };
}

#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $($msg:tt)*) => {

        let message = format!($($msg)*);
        $logger.debug().await.log_in_file(&message).await;
    };
}

#[macro_export]
macro_rules! log_trace {
    ($logger:expr, $($msg:tt)*) => {

        let message = format!($($msg)*);
        $logger.trace().await.log_in_file(&message).await;
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($logger:expr, $($msg:tt)*) => {

        let message = format!($($msg)*);
        $logger.critical().await.log_in_file(&message).await;
    };
}
