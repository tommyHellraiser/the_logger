# v0.5.0 - Initial release
- TheLogger supports configuration on startup and once it's already been instantiated.
- This logger can be configured to turn off every log element almost individually down to the microseconds if needed, 
except for the log text content (message itself to log).
- It uses tokio to lock the file writer, lazy_static to keep a global instance and chrono to fetch the datetime.
- Only one line to instantiate, and one line to execute a log.
- Support for 7 different log levels:
  - Verbose
  - Informational
  - Error
  - Warning
  - Debug
  - Trance
  - Critical
- Support to configure the length of the log location (file name, line, column) content maximum length, and also the 
log text content maximum length.
- All file writing is done using the std library.
- Global RwLock of the file writer to ensure each thread logs its message in the requested order, and to avoid
inconsistencies and errors when logging from multiple threads.
- Use TheLogger by passing it as a parameter to different functions, or by calling an instance of it.
- It's a singleton, so a single static reference will run through the whole program
- Tested locally with my personal projects, so ANY usage and bug reports will be greatly appreciated.