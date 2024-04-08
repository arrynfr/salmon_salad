use super::font::*;
use core::arch::asm;

#[repr(C, align(4))]
#[derive(Copy, Clone, Debug)]
pub struct Color {pub b: u16, pub g: u16, pub r: u16}

#[derive(Debug ,PartialEq, Clone)]
pub enum PixelFormat {
    BGR8,
    BGRX8,
    APL
}

#[derive(Debug, Clone)]
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
        assert!(addr as usize % 16 == 0, "Graphics buffer has to be 16 byte aligned!");
        assert!(str%16 == 0 || str%8 == 0 || str%4 == 0, "Graphics buffer stride needs to be 16|8|4 byte aligned! Got {} {}", sz, str);
        GraphicsBuffer {
            address: addr,
            size: sz,
            stride: str,
            horizontal_resolution: hr,
            vertical_resolution: vr,
            pixel_format: pf,
            bpp
        }
    }

    #[cfg(target_arch = "aarch64")]
    unsafe fn _clear_screen_arm(&self) {
        let fba = self.address;
        let end = self.size+fba as usize;
        let w: u64 = 0x0;
        if (end-fba as usize)%16 == 0 {
            asm!("1:","stp x2, x2, [x0], #16", "cmp x0, x1", "bne 1b", in("x0") fba, in("x1") end, in("x2") w);
        } else if (end-fba as usize)%8 == 0 {
            asm!("1:","str x2, [x0], #8", "cmp x0, x1", "bne 1b", in("x0") fba, in("x1") end, in("x2") w);
        } else if (end-fba as usize)%4 == 0 {
            asm!("1:","str w2, [x0], #4", "cmp x0, x1", "bne 1b", in("x0") fba, in("x1") end, in("x2") w);
        }
    }

    unsafe fn _clear_screen_generic(&self) {
        if self.size%16 == 0 {
            let fba: *mut u128 = self.address as *mut u128;
            for x in 0..self.size/16 {
                fba.add(x).write_volatile(0);
            }
        }
    }
    
    #[inline(always)]
    pub unsafe fn clear_screen(&self) {
        #[cfg(target_arch = "aarch64")]
        self._clear_screen_arm();
        #[cfg(not(target_arch = "aarch64"))]
        self._clear_screen_generic();
    }

    pub unsafe fn draw_pixel(&self, x: isize, y: isize, color: u32) {
        match self.pixel_format {
            PixelFormat::BGR8 => {
                let i_offset = x+(y*self.horizontal_resolution as isize);
                let offset: usize = i_offset as usize;
                if i_offset > 0 && offset < self.size {
                    let fba = self.address;
                    fba.wrapping_add(offset*self.bpp).write_volatile((color & 0xFF) as u8);
                    fba.wrapping_add(offset*self.bpp+1).write_volatile(((color >> 8) & 0xFF) as u8);
                    fba.wrapping_add(offset*self.bpp+2).write_volatile(((color >> 16) & 0xFF) as u8);
                }
            },
            PixelFormat::BGRX8 => {
                let i_offset = x+(y*self.horizontal_resolution as isize);
                let offset: usize = i_offset as usize;
                let fba = self.address as *mut u32;
                if i_offset > 0 && offset < self.size {
                    fba.wrapping_add(offset).write_volatile(color);
                }
            }
            PixelFormat::APL => {
                let i_offset = x+(y*self.horizontal_resolution as isize);
                let offset: usize = i_offset as usize;
                let fba = self.address as *mut u32;
                if i_offset > 0 && offset < self.size {
                    fba.wrapping_add(offset).write_volatile(color);
                }
            }
        }
    }

    pub unsafe fn draw_pixel_col(&self, x: isize, y: isize, color: Color) {
        let col=if self.pixel_format == PixelFormat::APL {
            ((color.r & 0x3FF) as u32) << 20 | ((color.g & 0x3FF) as u32) << 10 | ((color.b & 0x3FF) as u32)
        } else {
            ((color.r & 0xFF) as u32) << 16 | ((color.g & 0xFF) << 8) as u32 | ((color.b & 0xFF) as u32)
        };
        self.draw_pixel(x, y, col);
    }

    //TODO: finish
    pub unsafe fn draw_line(&self, start_x: isize, start_y: isize, stop_x: isize, stop_y: isize, color: Color) {
        let dx: isize = stop_x - start_x;
        let dy: isize = stop_y - start_y;
        let mut d: isize  = 2*dy - dx;
        let mut y: isize = start_y;
    
        for x in start_x..stop_x {
            self.draw_pixel_col(x, y, color);
            if d > 0 {
                y += 1;
                d -= 2*dx;
            }
            d += 2*dy
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
