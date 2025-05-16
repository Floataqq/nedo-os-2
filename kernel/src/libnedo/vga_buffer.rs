//! A bit of code for interacting with the VGA text buffer.
//! For now it is assumed that the buffer is 80 x 25.

use core::fmt;
use core::fmt::Write;
use core::fmt::Result;
use core::mem::transmute;
use core::default::Default;
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
    // TODO: maybe get rid of the transmutes...
    /// Construct a white-on-black color pair
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Construct a color pair from a background and foreground
    pub fn pair(bg: Color, fg: Color) -> Self {
        Self::new()
            .with_bg(bg)
            .with_fg(fg)
    }
    
    /// Get the background from a `ColorInfo`
    /// For now this and `fg` use transmutes because i'm too lazy
    pub fn bg(&self) -> Color {
        unsafe { transmute(self.0 >> 4) }
    }
    
    /// Get the foreground from a `ColorInfo`
    /// For now this and `bg` use transmutes because i'm too lazy
    pub fn fg(&self) -> Color {
        unsafe { transmute(self.0 & 0xF) }
    }
    
    /// Consume a `ColorInfo` to create one with changed background
    pub fn with_bg(mut self, color: Color) -> Self {
        self.0 = (self.0 & 0x0F) | ((color as u8) << 4);
        self
    }

    /// Consume a `ColorInfo` to create one with changed foreground
    pub fn with_fg(mut self, color: Color) -> Self {
        self.0 = (self.0 & 0xF0) | ((color as u8) & 0x0F);
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
    /// Create a white-on-black `Character` from a u8
    pub fn new(char: u8) -> Self {
        Self::with_color(char, ColorInfo::default())
    }
    
    /// Create a character from a u8 with any colors
    pub fn with_color(char: u8, color: ColorInfo) -> Self {
        Character {
            char,
            color
        }
    }
}

/// Address of the memory-mapped VGA text buffer
pub const VGA_BUFFER_START: usize = 0xb8000;
/// Default width of the VGA text buffer
pub const BUFFER_WIDTH: usize = 80;
/// Default width of the VGA text buffer
pub const BUFFER_HEIGHT: usize = 25;

#[derive(Debug, Clone)]
pub struct Writer {
    /// The first line that has free characters
    pub line: usize,
    /// The first **free* character on current line
    pub column: usize,
    /// The cursor color that characters will be outputted with
    pub color: ColorInfo,
}

impl Writer {
    /// Construct a `Writer` with it's cursor in the top left corner
    /// and the default color (white on black)
    pub fn new() -> Self {
        Writer {
            line: 0,
            column: 0,
            color: ColorInfo::default(),
        }
    }
    
    /// Consume a `Writer` to change it's cursor color
    pub fn color(mut self, color: ColorInfo) -> Self {
        self.color = color;
        self
    }

    /// Add a new line. If you call this while being on the last 
    /// line the cursor will go to the first line
    fn write_newline(&mut self) {
        // Overlow over the entire screen just loops right now
        self.column = 0;
        self.line = (self.line + 1) % BUFFER_HEIGHT;
    }
    
    /// Write a `Character` disregarding the cursor color
    pub fn write_character(&mut self, char: Character) {
        if self.column == BUFFER_WIDTH - 1 || char.char == b'\n' {
            self.write_newline();
            return;
        }
        unsafe {
            let addr = 
                (VGA_BUFFER_START as *mut Character).add(
                    self.line * BUFFER_WIDTH + self.column);
            *addr = char;
        }
        self.column += 1;
    }
    
    /// Write a `u8` with the cursor color
    pub fn write_byte(&mut self, char: u8) {
        self.write_character(
            Character::with_color(char, self.color));
    }
    
    /// Set the cursor color to `color`, clear the screen and 
    /// move to top left corner
    pub fn clear_color(&mut self, color: ColorInfo) {
        self.color = color;
        self.line = 0;
        self.column = 0;
        for i in 0..BUFFER_WIDTH * BUFFER_HEIGHT {
            self.write_byte(b' ');
        }
    }

    /// Clear the screen with the cursor color and move to
    /// top left corner
    pub fn clear(&mut self) {
        self.clear_color(self.color);
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

lazy_static! {
    #[doc(hidden)]
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

/// Just like print!():
/// ```rust
/// vga_print!("meow!")
/// ```
/// Will print out `meow!` with the set color
#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => (
        $crate::vga_buffer::_print(format_args!($($arg)*))
    );
}

/// Just like println!():
/// ```rust
/// vga_println!("meow!")
/// ```
/// Will print out `meow!` with the set color, and a new line
#[macro_export]
macro_rules! vga_println {
    () => ($crate::vga_print!("\n"));
    ($($arg:tt)*) => (
        $crate::vga_print!("{}\n", format_args!($($arg)*))
    );
}

/// Just like print!(), but with color:
/// ```rust
/// let col = ColorInfo::new()
///     .with_fg(Color::White)
///     .with_bg(Color::Red);
/// vga_color!(col, "meow!");
/// ```
/// Will write out `meow!`, with red background and white text
#[macro_export]
macro_rules! vga_color {
    ($color:expr, $($arg:tt)*) => (
        $crate::vga_buffer::_print_color(
            $color, format_args!($($arg)*))
    );
}

/// Just like println!(), but with color:
/// ```rust
/// let col = ColorInfo::new()
///     .with_fg(Color::White)
///     .with_bg(Color::Red);
/// vga_colorln!(col, "meow!");
/// ```
/// Will write out `meow!`, with red background and white text
/// and a new line
#[macro_export]
macro_rules! vga_colorln {
    ($color:expr, $($arg:tt)*) => (
        $crate::vga_buffer::_print_color($color, 
            format_args!("{}\n", $($arg)*))
    );
}

/// Set the cursor color for vga output
#[macro_export]
macro_rules! vga_setcolor {
    ($color:expr) => {
        $crate::vga_buffer::WRITER.lock().color = $color;
    };
}

/// Clear the screen with the specified color (or cursor color)
#[macro_export]
macro_rules! vga_clear {
    () => (
        $crate::vga_buffer::WRITER.lock().clear()
    );
    ($color:expr) => (
        $crate::vga_buffer::WRITER.lock().clear_color($color)
    );
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

#[doc(hidden)]
pub fn _print_color(color: ColorInfo, args: fmt::Arguments) {
    let mut lock = WRITER.lock();
    let old_color = lock.color;
    lock.color = color;
    lock.write_fmt(args).unwrap();
    lock.color = old_color;
}


