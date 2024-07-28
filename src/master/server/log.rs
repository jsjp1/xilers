use log::{Level, Metadata, Record, SetLoggerError};

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "[{}]({}:{}): {}",
                record.level(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args(),
            );
        }
    }

    fn flush(&self) {}
}
