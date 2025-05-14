//! A bit of code for interacting with the VGA text buffer.
//! For now it is assumed that the buffer is 80 x 25.

use core::fmt::Write;
use core::fmt::Result;
use core::mem::transmute;
use spin::Mutex;
use lazy_static::lazy_static;

/// 4-bit representation of the color. The "bright" 
/// background colors make text blink
#[derive(Debug)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15
}

/// A struct to hold background and foreground color
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct ColorInfo(u8);

impl Default for ColorInfo {
    fn default() -> Self {
        ColorInfo(0x0F)
    }
}

impl ColorInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pair(bg: Color, fg: Color) -> Self {
        Self::new()
            .with_bg(bg)
            .with_fg(fg)
    }

    pub fn bg(&self) -> Color {
        unsafe { transmute(self.0 >> 4) }
    }

    pub fn fg(&self) -> Color {
        unsafe { transmute(self.0 & 0xF) }
    }

    pub fn with_bg(mut self, color: Color) -> Self {
        self.0 |= (color as u8) << 4;
        self
    }

    pub fn with_fg(mut self, color: Color) -> Self {
        self.0 |= (color as u8) & 0xFF;
        self
    }
}

/// A character packs a codepoint and color information
#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct Character {
    pub char: u8,
    color: ColorInfo,
}

impl Character {
    pub fn new(char: u8) -> Self {
        Self::with_color(char, ColorInfo::default())
    }

    pub fn with_color(char: u8, color: ColorInfo) -> Self {
        Character {
            char,
            color
        }
    }
}

const VGA_BUFFER_START: usize = 0xb8000;
const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[derive(Debug, Clone)]
pub struct Writer {
    /// The first line that has free characters
    pub line: usize,
    /// The first **free* character on current line
    pub column: usize,
    pub color: ColorInfo,
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            line: 0,
            column: 0,
            color: ColorInfo::default(),
        }
    }

    pub fn color(mut self, color: ColorInfo) -> Self {
        self.color = color;
        self
    }

    /// Overlow over the entire screen just loops right now
    fn write_newline(&mut self) {
        self.column = 0;
        self.line = (self.line + 1) % BUFFER_HEIGHT;
    }
    
    /// Write a `Character` disregarding the cursor color
    pub fn write_character(&mut self, char: Character) {
        if self.column == BUFFER_WIDTH - 1 || char.char == b'\n' {
            self.write_newline()
        }
        let addr = VGA_BUFFER_START 
            + (self.line * BUFFER_WIDTH + self.column) * 2;
        unsafe { *(addr as *mut Character) = char }
        self.column += 1;
    }

    pub fn write_byte(&mut self, char: u8) {
        self.write_character(
            Character::with_color(char, self.color));
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {    
        for x in s.as_bytes() {
            self.write_byte(*x);
        }
        Ok(())
    }
}

/// A global writer instance for `print` and `println`
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

/// just like `print`, but uses the VGA buffer
#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        write!(*$crate::vga_buffer::WRITER.lock(), $($arg)*)
    };
}

/// just like `println`, but uses the VGA buffer
#[macro_export]
macro_rules! vga_println {
    ($($arg:tt)*) => {
        vga_print!($($arg)*, "\n")
    };
}


