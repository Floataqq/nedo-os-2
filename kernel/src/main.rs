#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use libnedo::vga_buffer::{Character, Color, ColorInfo, Writer};
use libnedo::{vga_color, vga_colorln, vga_print, vga_println, vga_clear};
use libnedo::vga_buffer::_print_color;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_println!("meow!");
    vga_clear!(ColorInfo::new().with_bg(Color::Red));
    vga_println!("meow!");
    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    vga_clear!();
    let col = ColorInfo::new()
        .with_fg(Color::White)
        .with_bg(Color::Red);
    vga_colorln!(col, "KERNEL PANIC!");
    if let Some(loc) = info.location() {
        vga_print!("{}/{}:{}: ", 
            loc.file(), loc.line(), loc.column());
    }
    vga_println!("{}", info.message());
    loop {}
}

