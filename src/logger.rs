use crate::sync::SpinLock;
use crate::{ansiterm, serial_println};

struct Logger {
    writer: SpinLock<LogSerialWriter>,
}

static LOGGER: Logger = Logger {
    writer: SpinLock::new(LogSerialWriter),
};

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            self.writer.lock_disable_interrupts().print_record(record);
        }
    }

    fn flush(&self) {}
}

struct LogSerialWriter;

impl LogSerialWriter {
    #[allow(clippy::unusead_self)]
    fn print_record(&self, record: &log::Record) {
        let color = match record.level() {
            log::Level::Error => ansiterm::Color::Red,
            log::Level::Warn => ansiterm::Color::Yellow,
            log::Level::Info => ansiterm::Color::Green,
            log::Level::Debug | log::Level::Trace => ansiterm::Color::White,
        };
        let color_code = ansiterm::AnsiEscapeSequence::SelectGraphicRendition(
            ansiterm::SelectGraphicRendition::ForegroundColor(color),
        );
        let clear = ansiterm::CLEAR_FORMAT;

        serail_println!("{color_code}[{}]{clear} {}", record.level(), record.args());
    }
}

pub(crate) fn init(){
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);
    log::debug!("Logging initialized");
}

pub(crate) fn force_unlock_logger() {
    unsafe {
        LOGGER.writer.force_unlock();
    }
}
