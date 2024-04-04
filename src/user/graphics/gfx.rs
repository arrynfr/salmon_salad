use core::mem::transmute;
use super::font::*;

#[repr(C, align(4))]
#[derive(Copy, Clone, Debug)]
pub struct Color {pub b: u16, pub g: u16, pub r: u16}
pub struct Vec2 {x: usize, y: usize}

#[derive(Debug ,PartialEq)]
pub enum PixelFormat {
    BGR8,
    BGRX8,
    APL
}

#[derive(Debug)]
pub struct GraphicsBuffer {
    address: *mut u8,
    pub size: usize,
    pub stride: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: PixelFormat,
    pub bpp: usize
}

impl GraphicsBuffer {
    pub fn new(addr: *mut u8, sz: usize, str: u32, hr: u32, vr: u32, pf: PixelFormat, bpp: usize) -> Self {
        GraphicsBuffer {
            address: addr,
            size: sz,
            stride: str,
            horizontal_resolution: hr,
            vertical_resolution: vr,
            pixel_format: pf,
            bpp: bpp
        }
    }

    pub unsafe fn clear_screen(&self) {
        let fba: *mut u128 = self.address as *mut u128;
        let col_128: u128 = 0 as u128;
        if self.size%16 == 0 {
            for x in 0..self.size/16 {
                fba.add(x).write_volatile(col_128);
            }
        } else {
            for x in 0..self.size {
                (fba as *mut u8).add(x).write_volatile(0);
            }
        }
    }

    pub unsafe fn draw_pixel(&self, x: isize, y: isize, color: u32) {
        let stride = self.stride;
        match self.pixel_format {
            PixelFormat::BGR8 => {
                let i_offset = x+(y*stride as isize)/self.bpp as isize;
                let offset: usize = i_offset as usize;
                if i_offset > 0 && offset < self.size {
                    let fba = self.address as *mut u8;
                    fba.wrapping_add(offset*self.bpp).write_volatile((color & 0xFF) as u8);
                    fba.wrapping_add(offset*self.bpp+1).write_volatile(((color >> 8) & 0xFF) as u8);
                    fba.wrapping_add(offset*self.bpp+2).write_volatile(((color >> 16) & 0xFF) as u8);
                }
            },
            PixelFormat::BGRX8 => {
                let i_offset = x+(y*stride as isize);
                let offset: usize = i_offset as usize;
                let fba = self.address as *mut u32;
                if i_offset > 0 && offset < self.size {
                    fba.wrapping_add(offset).write_volatile(color);
                }
            }
            PixelFormat::APL => {
                let i_offset = x+(y*stride as isize);
                let offset: usize = i_offset as usize;
                let fba = self.address as *mut u32;
                if i_offset > 0 && offset < self.size {
                    fba.wrapping_add(offset).write_volatile(color);
                }
            }
        }
    }

    pub unsafe fn draw_pixel_col(&self, x: isize, y: isize, color: Color) {
        let col;
        if self.pixel_format == PixelFormat::APL {
            col = 0 | ((color.r & 0x3FF) as u32) << 20 | ((color.g & 0x3FF) as u32) << 10 | ((color.b & 0x3FF) as u32) << 0;
        } else {
            col = ((color.r & 0xFF) as u32) << 16 | ((color.g & 0xFF) << 8) as u32 | ((color.b & 0xFF) as u32);
        }
        self.draw_pixel(x, y, col);
    }

    pub unsafe fn draw_line(&self, start_x: isize, start_y: isize, stop_x: isize, stop_y: isize, color: Color) {
        let dx: isize = stop_x - start_x;
        let dy: isize = stop_y - start_y;
        let mut d: isize  = 2*dy - dx;
        let mut y: isize = start_y;
    
        for x in start_x..stop_x {
            self.draw_pixel_col(x, y, color);
            if d > 0 {
                y = y +1;
                d = d - 2*dx;
            }
            d = d + 2*dy
        }
    }

    pub unsafe fn draw_circle(&self, center: (usize, usize), radius: usize, color: Color) {
        let (cx, cy) = center;
        let s = self.stride as usize;
        let x_min = cx.saturating_sub(radius);
        let y_min = cy.saturating_sub(radius);
        let x_max = (cx + radius).min(s);
        let y_max = (cy + radius).min(self.size/s);
        
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                // Check if the current pixel is within the radius of the circle
                if (x as isize - cx as isize).pow(2) + (y as isize - cy as isize).pow(2) <= (radius as isize).pow(2) {
                    self.draw_pixel_col(x.try_into().unwrap(), y.try_into().unwrap(), color);
                }
            }
        }
    }

    pub unsafe fn draw_rectangle(&self, x: isize, y: isize, w: isize, h: isize, color: Color) {
        for off_x in 0..w {
            for off_y in 0..h {
                self.draw_pixel_col(x+off_x, y+off_y, color);
            } 
        }
    }

    pub unsafe fn draw_character(&self, x: isize, y: isize, ch: char, color: Color, font_size: isize) {
        let font_char = FONT[ch as usize];
        for row in 0..8 {
            let font_char_row = font_char[row as usize];
            for px in 0..8 {
                if font_char_row & (1 << px) != 0 {
                    self.draw_rectangle(x+px*font_size, y+row*font_size, font_size, font_size, color);
                }
            }
        }
    }

    pub unsafe fn draw_string(&self, x: isize, y: isize, string: &str, color: Color, font_size: isize) {
        let mut line = 0;
        let mut pos = 0;
        for character in string.chars() {
            if character == '\n' {
                line += 1;
                pos = 0;
            } else {
                self.draw_character(x+(font_size*8*pos as isize), y+(font_size*8*line), character, color, font_size);
                pos += 1;
            }
        }
    }
}
