//! Contains the multiboot2 header and the `_start` function.
//! Also houses all the temporary infastructure (e.g. the stack)
//! used to get to `kernel_main`

use core::fmt::Write;
use core::mem::size_of;
use core::arch::asm;
use libnedo::multiboot2::MultibootHeader;
use libnedo::vga_buffer::Writer;

/// The multiboot header that the kernel uses for things
#[used]
#[unsafe(link_section = ".multiboot_header")]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: 0xe85250d6, // multiboot 2
    arch: 0,           // protected mode i386
    length: size_of::<MultibootHeader>() as u32,
    checksum: 0x100000000_u64.overflowing_sub(
        0xe85250d6 + size_of::<MultibootHeader>() as u64
    ).0 as u32,
    header_type: 0,
    flags: 0,
    size: 8,
};

/// Temporary 4096 byte stack for all the initial routines
#[used]
#[unsafe(link_section = ".bss")]
static mut STACK: [u8; 4096] = [b'A'; 4096];

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // need to do this as soon as possible
    // before the compiler tries to do anything
    unsafe {
        let stack_top = (&raw const STACK) as usize;
        asm!("mov rsp, rdi", in ("rdi") stack_top);
        asm!("mov rbp, rdi", in ("rdi") stack_top);
    }
    start_kernel();
    loop {}
}

pub fn start_kernel() {
    let mut w = Writer::new();
    // if the length is >= 16 bytes, the compiler
    // stops unrolling the cycle and everything somehow breaks
    w.write_str("0123456789abcdef").unwrap();
}

