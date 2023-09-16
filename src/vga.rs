enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

// Add row: usize and column: usize
// Add buffer: *mut u8,
pub struct Vga {
    row: isize,
    column: isize,
    buffer: *mut u16,
}

// For the buffer, each pixel is 16 bytes:
/*
16           12   11          8   7                             0
+---------------+---------------+-------------------------------+
|               |               |                               |
|  Background   |  Foreground   |           Character           |
|               |               |                               |
+---------------+---------------+-------------------------------+
*/

// Create new method which returns Vga where row is 0 and column is 0
impl Vga {
    pub fn new() -> Vga {
        // let vga_buffer = 0xB8000 as *mut u16;
        let vga_buffer = 0xB8000 as *mut u16;
        Vga { row: 0, column: 0, buffer: vga_buffer }
    }

    // Create a clear screen method
    // - It should take &mut self
    pub fn clear_screen(&mut self) {
        // The screen is 80x25
        for pixel in 0..(80 * 25) {
            unsafe {
                *self.buffer.offset(pixel) = 0;
            }
        }
    }

    // Add method called write
    pub fn write(&mut self, letter: char) {
        unsafe {
            *self.buffer.offset(self.column) = 0x0F << 8 | letter as u16;
        }
        self.column += 1;
    }

    // Add method called write_str, which takes input: &str
    pub fn write_str(&mut self, input: &str) {
        // For each character in writing, call self.write with that character;
        for current_character in input.chars() {
            self.write(current_character);
        }
    }
    // self.write_str("Hello, there!");
}