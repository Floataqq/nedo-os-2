#[repr(C, packed)]
pub struct MultibootHeader {
    pub magic: u32,
    pub arch: u32,
    pub length: u32,
    pub checksum: u32,
    /* optional tags here */
    pub header_type: u16,
    pub flags: u16,
    pub size: u32,
}
