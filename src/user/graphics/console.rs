use core::{fmt::{Result,Write}, sync::atomic::AtomicBool};
use crate::user::graphics::gfx::GraphicsBuffer;

use super::gfx::Color;

static GFX_LOCK: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub struct GfxConsole<'a> {
    width: u32,
    height: u32,
    font_scale: u32,
    font_size: u32,
    font_color: Color,
    background_color: Color,
    cursor: (u32, u32),
    gfx_buffer: &'a GraphicsBuffer,
}

impl<'a> Drop for GfxConsole<'a> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<'a> GfxConsole<'a> {
    pub fn new(fs: u32, gb: &'a GraphicsBuffer) -> Self {
        let mut c = GfxConsole {
            width: gb.horizontal_resolution/(8*fs),
            height: gb.vertical_resolution/(8*fs),
            font_scale: fs,
            font_size: fs*8,
            font_color: Color { b: 0x3FF, g: 0x3FF, r: 0x3FF },
            background_color: Color { b: 0, g: 0, r: 0 },
            cursor: (0,0),
            gfx_buffer: gb
        };
        c.clear();
        c
    }

    pub fn _get_width(&self) -> u32 {
        self.width
    }

    pub fn _get_height(&self) -> u32 {
        self.height
    }

    pub fn _get_font_scale(&self) -> u32 {
        self.font_scale
    }

    pub fn _get_font_size(&self) -> u32 {
        self.font_size
    }

    pub fn _get_cursor(&self) -> (u32, u32) {
        self.cursor
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
    
    pub fn set_font_color(&mut self, color: Color) {
        self.font_color = color;
    }

    pub fn set_font_scale(&mut self, scale: u32) {
        if scale > 0 && scale <= 10 {
            self.font_scale = scale;
            self.font_size = 8*scale;
            self.width = self.gfx_buffer.horizontal_resolution/self.font_size;
            self.height = self.gfx_buffer.vertical_resolution/self.font_size;
            self.clear();
        }
    }

    pub fn _aquire() {
        while GFX_LOCK.compare_exchange(false,
            true,
            core::sync::atomic::Ordering::SeqCst,
            core::sync::atomic::Ordering::SeqCst).is_err() {}
    }

    pub fn _release() {
        GFX_LOCK.compare_exchange(true,
            false,
            core::sync::atomic::Ordering::SeqCst,
            core::sync::atomic::Ordering::SeqCst)
            .expect("Trying to release unheld lock: gfx_lock");
    }

    pub fn write(&mut self, st: &str) {
        unsafe {
            self.gfx_buffer.draw_rectangle((self.cursor.0*self.font_size) as isize, (self.cursor.1*self.font_size) as isize, 
            self.font_size as isize, self.font_size as isize, self.background_color);
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
                if c == '\u{8}' {
                    
                    self.gfx_buffer.draw_rectangle((self.cursor.0*self.font_size) as isize, (self.cursor.1*self.font_size) as isize, 
                    self.font_size as isize, self.font_size as isize, self.background_color);
                    self.cursor.0 -= 1;
                    continue;
                }
                if self.cursor.0 >= self.width {
                    self.cursor.0 = 0;
                    self.cursor.1 += 1;
                }
                if self.cursor.1 >= self.height {
                    self.clear();
                }
                self.gfx_buffer.draw_rectangle((self.cursor.0*self.font_size) as isize, (self.cursor.1*self.font_size) as isize, 
                self.font_size as isize, self.font_size as isize, self.background_color);
                self.gfx_buffer.draw_character((self.cursor.0*self.font_size) as isize,
                (self.cursor.1*self.font_size) as isize,
                c, self.font_color, self.font_scale as isize);
                self.cursor.0 += 1;
            }
            self.gfx_buffer.draw_rectangle((self.cursor.0*self.font_size) as isize, (self.cursor.1*self.font_size) as isize, 
            self.font_size as isize, self.font_size as isize, Color { b: 0x3FF, g: 0x3FF, r: 0x3FF });
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

impl<'a> Write for GfxConsole<'a> {
    fn write_str(&mut self, string: &str) -> Result {
        self.write(string);
        Ok(())
    }
}

#[macro_export]
macro_rules! gfx_print {
    ($($arg:tt)*) => {
        <$crate::user::graphics::gfx::DbgWriter>::aquire_lock();
        let _ = <$crate::print::DbgWriter as core::fmt::Write>::write_fmt(
            &mut $crate::print::DbgWriter,
            format_args!($($arg)*));
        <$crate::print::DbgWriter>::release_lock();
    }
}