use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
use core::fmt;

lazy_static! {
    static ref SERIAL1: Mutex<SerialPort> = {
        // 0x3F8 = standard COM1 port
        let mut port = unsafe { SerialPort::new(0x3F8) };
        port.init();
        Mutex::new(port)
    };
}

pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    SERIAL1.lock().write_fmt(args).ok();
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => { $crate::serial_print!("\n"); };
    ($fmt:expr) => { $crate::serial_print!(concat!($fmt, "\n")); };
    ($fmt:expr, $($arg:tt)*) => { $crate::serial_print!(concat!($fmt, "\n"), $($arg)*); };
} 