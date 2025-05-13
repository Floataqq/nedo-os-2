#![no_std]
#![no_main]

mod boot;

use core::panic::PanicInfo;
use core::fmt::Write;
use libnedo::vga_buffer::Writer;

#[allow(dead_code)]
fn kernel_main() {}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    let mut w = Writer::new();
    write!(w, "Kernel panic at ");
    match _info.location() {
        Some(l) => write!(w, "({}, {})", l.line(), l.column()),
        None => writeln!(w, "an unknown loc"),
    };
    loop {}
}
