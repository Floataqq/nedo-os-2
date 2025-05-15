#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use libnedo::vga_buffer::Writer;
use libnedo::{vga_println, vga_print};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_println!("Hello!");
    vga_println!("World!");
    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

