#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use libnedo::vga_buffer::Writer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut w = Writer::new();
    write!(w, "meowmeowmeowmeow");
    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

