use core::arch::asm;

#[inline]
pub fn hlt() {
    unsafe { 
        asm!("hlt", options(nomem, nostack, preserves_flags));
    }
}

#[inline]
pub fn nop() {
    unsafe {
        asm!("nop", options(nomem, nostack, preserves_flags));
    }
}

