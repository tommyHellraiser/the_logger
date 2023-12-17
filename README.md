[![Crates.io](https://img.shields.io/crates/v/the_logger?logo=rust)](https://crates.io/crates/the_logger)
![Crates.io](https://img.shields.io/crates/l/the_logger/0.5.0)
[![Rust](https://github.com/tommyHellraiser/the_logger/actions/workflows/compile.yml/badge.svg)](https://github.com/tommyHellraiser/the_logger/actions/workflows/compile.yml)
[![rustc_version]][Rust 1.72]
[![the_logger]][the_logger_docs]


[rustc_version]: https://img.shields.io/badge/rustc_version-1.31+-lightgray.svg
[Rust 1.72]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[the_logger]: https://img.shields.io/badge/the__logger-docs.rs-green?logo=docs.rs

[the_logger_docs]: https://docs.rs/the_logger/0.5.0/the_logger/

# the_logger;

This crate was created with the purpose of making logging easier to the point where you need only two lines
to create a log file. But besides being easy to use and spawn, it's also really customizable down to the 
microseconds.

This logging system is based on the std library to write to files, and uses tokio only for globally locking
the file writer, lazy_static to keep the static reference of the file writer and chrono to fetch the date
and time for the logs. Optionally (coming in a near future version), you can configure its settings by
creating a json file and setting the config in it.

## Features
- Really easy to call and use.
- Very customizable, with more personalization always coming little by little.
- Lightweight, with only 3 dependencies with the default features (lazy_static, tokio and chrono).
- Accessible anywhere in the code, no need to pass by reference.
- Configurable parameters on startup and on the run.

## How to use
As mentioned, base usage is really simple, no configuration is required to log in its default settings. An async 
function is required to call the logger:
````rust
use the_logger::{log_warning, TheLogger};

async fn init_logger(thread_id: u8) {
    let logger: &TheLogger = TheLogger::instance();
    log_warning!(logger, "This is a warning emitted by thread {}", thread_id);
}
````

As shown in the example above, only 2 lines of code were necessary to set up the logger and log a message. 
There are a couple of ways to execute the logging but the least verbose one is via a macro call. Another 
way of doing it is by:

````rust
use the_logger::TheLogger;

async fn init_logger(thread_id: u8) {
    TheLogger::instance()
        .warning().await
        .log_in_file(
            (file!(), line!(), column!()
        ), "This is another logging example").await;
}
````
The latter way reduces the amount of lines to 1, but at the expense of making the call a lot more verbose and least 
intuitive, missing the purpose of this crate.

As seen in the first example, the log_X!() macros have built-in support for the format!() macro, no need to allocate 
an extra variable to build the message. The first parameter of the macro needs to be the logger element, and the rest
of them can be used as built-in String formatter, supporting all the format!() macro features:

````rust
use the_logger::{log_warning, TheLogger};

async fn init_logger(thread_id: u8) {
    let logger: &TheLogger = TheLogger::instance();
    //  This log...
    log_warning!(logger, "This is a log by thread {}", thread_id);
    
    //  ...is equivalent content-wise to this log
    let msg = format!("This is a log by thread {}", thread_id);
    log_warning!(logger, msg);
}
````

There are 2 main ways to use this logger: passing the instance by reference, or calling an instance from anywhere you 
want to:

````rust
use the_logger::{log_info, TheLogger};

async fn parent_function() {
    let logger: &TheLogger = TheLogger::instance();
    
    child_function_one(logger).await;
    
    child_function_two();
}

async fn child_function_one(logger: &TheLogger) {
    
    log_info!(
        logger, 
        "This will trigger an Informational log by passing the logger instance as reference"
    );
    
}

async fn child_function_two() {
    let logger: &TheLogger = TheLogger::instance();
    
    log_info!(
        logger, 
        "This will trigger an Informational log by calling an instance of the logger without parameters"
    );
}
````
This is possible due to the use of tokio's RwLock feature mentioned at the beginning. This method decided to avoid 
inconsistencies and errors when writing to the log files. This way, no matter how many threads are running, 
the_logger crate will ensure the log lines are written one at a time, and in the order they were called.

## Log files
All the files generated will be placed inside the project using this crate, in the ``logs/`` folder with the 
name: ``"Log <year>-<month>-<day>.log"``

In future versions you'll be able to configure the name (date will always be present for better files organization), the
save path, the file format, and you'll also be able to delete files older than a configurable amount of days.


## Log levels
There are 7 log levels, that aren't all actually levels, but rather categories. These are:
- Verbose
- Informational
- Error
- Warning
- Debug
- Trace
- Critical

Each one of them may serve different purposes in your code, and are present for you to use them as you see fit.

To invoke each one of them, you can simply use the defined macro calls:
````rust
use the_logger::TheLogger;
use the_logger::{log, log_info, log_error, log_warning, log_debug, log_trace, log_critical};

async fn init_logger(thread_id: u8) {
    let logger: &TheLogger = TheLogger::instance();
    
    log!(logger, "This call will trigger a Verbose log");
    log_info!(logger, "This call will trigger an Informational log");
    log_error!(logger, "This call will trigger an Error log");
    log_warning!(logger, "This call will trigger a Warning log");
    log_debug!(logger, "This call will trigger a Debug log");
    log_trace!(logger, "This call will trigger a Trace log");
    log_critical!(logger, "This call will trigger a Critical log");
}
````

## Configuration
As mentioned in the beginning, this logger is very customizable. You can show and hide almost all of its elements:
- Years in the date
- Months in the date
- Days in the date
- Hours in the time
- Minutes in the time
- Seconds in the time
- Milliseconds in the time
- Microseconds in the time
- Log level
- Log location (file, line and column)

The default configuration is to show every of the aforementioned elements, except for the column in the file location
options.

You can also configure the maximum length of the location and content text. Location text content is limited to 60 
characters by default, and the log content itself, to 300.

There are 2 ways of configuring this logger: on startup and on the run (will be adding configuration by json file in 
the future). To configure on startup:
````rust
use the_logger::{TheLogger, TheLoggerConfig};

async fn config_logger() {
    let logger_config = TheLoggerConfig::default()
        .hide_millisecs()               //  This will hide both the milliseconds and the microseconds
        .hide_level()                   //  This will hide the log level
        .hide_location()                //  This will hide the file name, line and column
        .location_content_length(100)   //  This will set the maximum location content text length to 100
        .log_content_length(1000);      //  This will set the maximum log content text length to 1000
    //  Feel free to explore the documentation for more configuration options
    
    let logger: &TheLogger = TheLogger::instance().config_logger(logger_config).await;
    //  The logger is now configured with the options set above
}
````

This configuration method is recommended only at startup, since sending the TheLoggerConfig struct instance to the 
``.config_logger()`` will overwrite the current contents of the configuration. For on-the-go configuration, there's 
a more effective way of doing it:

````rust
use the_logger::{TheLogger, TheLoggerConfig};

async fn config_logger() {
    //  Logger instanced at start
    let logger: &TheLogger = TheLogger::instance();
    
    //  Logger configured on the go
    logger.hide_millisecs().await
        .hide_level().await
        .hide_location().await
        .location_content_length(100).await
        .log_content_length(1000).await;
    
    //  The configuration applied is the same as in the example above
}
````

## Log content examples
In the following examples, every parameter was left to its default vale, except the location content length, which was 
set to 20 characters:
````text
2023-12-17 01:51:10.677106	[VERBOSE]	@src\main.rs: 37    Log from thread 10
2023-12-17 01:51:10.677219	[WARNING]	@src\main.rs: 37    Log from thread 17
2023-12-17 01:51:10.677343	[TRACE]		@src\main.rs: 34    Log from thread 2
2023-12-17 01:51:10.702689	[CRITICAL]	@src\main.rs: 28    Log from thread 0
2023-12-17 01:51:10.810655	[ERROR]		@src\main.rs: 31    Log from thread 1
2023-12-17 01:51:11.367695	[INFO]		@src\main.rs: 34    Log from thread 2
2023-12-17 01:51:11.538744	[DEBUG]		@src\main.rs: 40    Log from thread 4
````

The previous log lines were selected out of approximately 350 lines generated by 30 threads. They were not extracted as 
a block, since some of the threads had longer sleep times than others, causing repetitions, so I had to select some of 
these lines.

The following is an example of a log line with all default settings:
````text
2023-12-17 01:55:43.144632	[WARNING]	@src\main.rs: 43                                            Log from thread 26
````

You can see how it becomes useful to be able to hide certain elements and configure the length of the location content 
and/or the log text content. For the purpose of better readability, we'll use a location content length of 20 chars
moving forward. Now, if we remove the log level we get:
````text
2023-12-17 01:58:57.874425		@src\main.rs: 39    Log from thread 24
````

Next, we'll also remove the milliseconds, which will inherently remove the microseconds as well:
````text
2023-12-17 01:59:57		@src\main.rs: 37    Log from thread 2
````

But you might have a moment by moment tracking of your program or service, and the log date might not be useful, only
the full timestamp, level, location and content, in that case:
````text
02:01:52.509640	[CRITICAL]	@src\main.rs: 32    Log from thread 0
````

You might also have a small program that has a single file and the error is very descriptive, you can configure 
it to get:
````text
02:02:54.830615	[INFO]		Log from thread 16
````

What if you only want the log level and the content because your error already includes date, time, and location? 
You can do that too:
````text
[CRITICAL]	Log from thread 14
````

And finally, if ouy want to hide everything, except the log text content:
````text
Log from thread 25
````

The only parameter you can't hide is the log text content, since it's the minimum requirement for a log.

## Some testing and conclusions

The ability to disable elements to log into the file comes with two major benefits: faster program execution due to the 
reduced amount of data that needs to be written in the file, and related to the amount of data is the file size.

To test the file size, I ran a program with 100 threads for two minutes in the full configuration (default parameters)
and with minimal configuration (only the log text content), and the file sizes were:
- Around 1116KB for the full configuration
- Around 177KB for the minimal configuration

Which means the file size was reduced by approximately 84% by leaving only the minimum required data. Of course, that 
amount of threads introduced bottlenecks in file writing because of the time they had to wait for the lock to be released,
but the size ratio remained constant across the different tests nevertheless. To ensure this was the case, I reduced 
the amount of threads to 30, with the same amount of time, and the results were:
- Around 792KB for the full configuration
- Around 124KB for the minimal configuration

Which gives the same full_size/minimal_size ratio.

## Future features and bug reports
As I mentioned across this readme file, I have in mind some improvements and features, that include:
- Configuration by creating a json file in a folder, yet to be determined
- Configuration of the log file path
- Configuration of the log file name
- Configuration of some separators
- Configuration of the log level enclosure -> [ERROR] vs {ERROR} vs (ERROR) vs ERROR vs |ERROR|, etc

If you have any suggestions, bugs to report, or any other kind of feedback, I'll leave some contact channels ion the 
next section.

## Wrap-up
That sums this crate up, I hope you find it useful, and if you have any questions, suggestions or reports, feel free
to contact me at:

email: ``nacho.ponce25@gmail.com``

telegram: ``@tommyHellraiser``

Or submit an issue in the GitHub repository:
https://github.com/tommyHellraiser/the_logger

Thank you for reading, and have a great one!