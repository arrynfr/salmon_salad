use core::fmt::{Result,Write};
use crate::user::graphics::gfx::GraphicsBuffer;

use super::gfx::Color;

#[derive(Debug)]
pub struct GfxConsole {
    width: usize,
    height: usize,
    font_scale: usize,
    font_size: usize,
    cursor: (usize, usize),
    gfx_buffer: GraphicsBuffer
}

impl GfxConsole {
    pub fn new(w: usize, h: usize, fs: usize, gb: GraphicsBuffer) -> Self {
        GfxConsole {
            width: w,
            height: h,
            font_scale: fs,
            font_size: fs*8,
            cursor: (0,0),
            gfx_buffer: gb
        }
    }

    pub fn _get_width(self) -> usize {
        self.width
    }

    pub fn _get_height(self) -> usize {
        self.height
    }

    pub fn _get_font_scale(self) -> usize {
        self.font_scale
    }

    pub fn _get_font_size(self) -> usize {
        self.font_size
    }

    pub fn _get_cursor(self) -> (usize, usize) {
        self.cursor
    }

    pub fn write(&mut self, st: &str) {
        unsafe {
            self.gfx_buffer.draw_rectangle((self.cursor.0*self.font_size) as isize, (self.cursor.1*self.font_size) as isize, 
            (self.font_size*st.len()) as isize, self.font_size as isize, Color { b: 0, g: 0, r: 0 });    
            for c in st.chars() {
                if c == '\r' {
                    self.cursor.0 = 0;
                    continue;
                }
                if c == '\n' {
                    self.cursor.0 = 0;
                    self.cursor.1 += 1;
                    continue;
                } 
                if self.cursor.0 >= self.width {
                    self.cursor.0 = 0;
                    self.cursor.1 += 1;
                }
                if self.cursor.1 >= self.height {
                    self.clear();
                }
                self.gfx_buffer.draw_character((self.cursor.0*self.font_size) as isize,
                (self.cursor.1*self.font_size) as isize,
                c, Color{r: 0x3ff, g: 0x3ff, b: 0x3ff}, self.font_scale as isize);
                self.cursor.0 += 1;
            }
        }
    }

    pub fn writeln(&mut self, st: &str) {
        self.write(st);
        self.write("\r\n");
    }

    pub fn clear(&mut self) {
        self.cursor.0 = 0;
        self.cursor.1 = 0;
        unsafe {
            self.gfx_buffer.clear_screen();
        }
    }
}

impl Write for GfxConsole {
    fn write_str(&mut self, string: &str) -> Result {
        self.write(string);
        Ok(())
    }
}
