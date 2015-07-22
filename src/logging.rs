// Logging configuration


extern crate log;


struct NormalLogger;
impl log::Log for NormalLogger {
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= log::LogLevel::Warn
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{}", record.args());
        }
    }
}


struct DebugLogger;
impl log::Log for DebugLogger {
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= log::LogLevel::Trace
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.metadata()) {
            println!(
                "{level} ({file}:{line}): {message}",
                level=record.level(),
                file=record.location().file(),
                line=record.location().line(),
                message=record.args()
            );
        }
    }
}


pub fn configure_logging(debug: bool) {
    let result = if debug {
        log::set_logger(
            |max_log_level| {
                max_log_level.set(log::LogLevelFilter::Trace);
                Box::new(DebugLogger)
            }
        )
    } else {
        log::set_logger(
            |max_log_level| {
                max_log_level.set(log::LogLevelFilter::Warn);
                Box::new(NormalLogger)
            }
        )
    };

    if let Err(_) = result {
        unreachable!()
    }
}
