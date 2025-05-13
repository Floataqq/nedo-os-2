//! Contains the multiboot2 header and the `_start` function.
//! Also houses all the temporary infastructure (e.g. the stack)
//! used to get to `kernel_main`

use core::mem::size_of;
use core::arch::asm;
use libnedo::multiboot2::MultibootHeader;

pub mod long_mode_init;

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
static mut STACK: [u8; 4096] = [0; 4096];

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // need to do this as soon as possible
    // before the compiler tries to do anything
    // TODO: everything somehow works without it,
    // gotta research
    /*
    unsafe {
        let stack_top = (&raw const STACK) as usize;
        asm!("mov esp, edi", in ("edi") stack_top);
    }
    */
    start_kernel();
    loop {}
}

pub fn start_kernel() {
    long_mode_init::try_long_mode();
}

