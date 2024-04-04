use crate::user::graphics::gfx::GraphicsBuffer;

use super::gfx::Color;

#[derive(Debug)]
pub struct GfxConsole {
    pub width: usize,
    pub height: usize,
    pub font_scale: usize,
    pub cursor: (usize, usize),
    pub gfx_buffer: GraphicsBuffer
}

impl GfxConsole {
    pub fn new(w: usize, h: usize, fs: usize, gb: GraphicsBuffer) -> Self {
        GfxConsole {
            width: w,
            height: h,
            font_scale: fs,
            cursor: (0,0),
            gfx_buffer: gb
        }
    }

    pub fn write(&mut self, st: &str) {
        unsafe {
            for c in st.chars() {
                if c == '\n' || self.cursor.0 >= self.width {
                    self.cursor.0 = 0;
                    self.cursor.1 += 1;
                    if c == '\n' {
                        continue;
                    } 
                }
                if self.cursor.1 >= self.height {
                    self.clear();
                }
                self.gfx_buffer.draw_character((self.cursor.0*8*self.font_scale) as isize,
                (self.cursor.1*8*self.font_scale) as isize,
                c, Color{r: 0x3ff, g: 0x3ff, b: 0x3ff}, self.font_scale as isize);
                self.cursor.0 += 1;
            }
        }
    }

    pub fn clear(&mut self) {
        self.cursor.0 = 0;
        self.cursor.1 = 0;
        unsafe {
            self.gfx_buffer.clear_screen();
        }
    }
}