
use core::fmt::Write;

use spin::Mutex;

use crate::VIRT_ADDR_START;

const UART_ADDR: usize = 0x01FE001E0 | VIRT_ADDR_START;

static COM1: Mutex<Uart> = Mutex::new(Uart::new(UART_ADDR));

pub struct Uart {
    base_address: usize,
}

impl Uart {
    pub const fn new(base_address: usize) -> Self {
        Uart { base_address }
    }

    pub fn putchar(&mut self, c: u8) {
        let ptr = self.base_address as *mut u8;
        loop {
            unsafe {
                let c = ptr.add(5).read_volatile();
                if c & (1 << 5) != 0 {
                    break;
                }
            }
        }
        unsafe {
            ptr.add(0).write_volatile(c);
        }
    }

    pub fn getchar(&mut self) -> Option<u8> {
        let ptr = self.base_address as *mut u8;
        unsafe {
            if ptr.add(5).read_volatile() & 1 == 0 {
                // The DR bit is 0, meaning no data
                None
            } else {
                // The DR bit is 1, meaning data!
                Some(ptr.add(0).read_volatile())
            }
        }
    }
}
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            self.putchar(c);
        }
        Ok(())
    }
}

/// Writes a byte to the console.
pub fn console_putchar(c: u8) {
    // let mut uart = COM1.lock();
    // match c {
    //     b'\n' => {
    //         uart.putchar(b'\r');
    //         uart.putchar(b'\n');
    //     }
    //     c => uart.putchar(c),
    // }
    COM1.lock().putchar(c)
}

pub fn write_fmt(args: core::fmt::Arguments) {
    use core::fmt::Write;
    COM1.lock().write_fmt(args).unwrap();
}

/// Reads a byte from the console, or returns [`None`] if no input is available.
pub fn console_getchar() -> u8 {
    COM1.lock().getchar().unwrap_or(u8::MAX)
}