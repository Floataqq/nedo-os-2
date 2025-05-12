use core::mem::size_of;

#[repr(C, packed)]
pub struct MultibootHeader {
    pub magic: u32,
    pub arch: u32,
    pub length: u32,
    pub checksum: u32,
    /* optional tags here */ 
    pub header_type: u16,
    pub flags: u16,
    pub size: u32
}

#[used]
#[unsafe(link_section = ".multiboot_header")]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: 0xe85250d6, // multiboot 2
    arch: 0,           // protected mode i386
    length: size_of::<MultibootHeader>() as u32,
    checksum: (0x100000000 - 
        (0xe85250d6 + size_of::<MultibootHeader>())) as u32,
    header_type: 0,
    flags: 0,
    size: 8
};

