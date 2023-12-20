use qemu_print::qemu_println;

static LOGGER: MyLogger = MyLogger;

struct MyLogger;
impl log::Log for MyLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        qemu_println!("[{}] {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_logger(&LOGGER).unwrap();
}
