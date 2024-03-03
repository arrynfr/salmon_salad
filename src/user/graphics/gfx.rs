use core::mem::transmute;

#[repr(C, align(4))]
#[derive(Copy, Clone, Debug)]
pub struct Color {pub b: u8, pub g: u8, pub r: u8}
pub struct Vec2 {x: usize, y: usize}

pub struct GraphicsBuffer {
    address: *mut u8,
    pub size: usize,
    pub stride: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: u32,
}

impl GraphicsBuffer {
    pub fn new(addr: *mut u8, sz: usize, str: u32, hr: u32, vr: u32, pf: u32) -> Self {
        GraphicsBuffer {
            address: addr,
            size: sz,
            stride: str,
            horizontal_resolution: hr,
            vertical_resolution: vr,
            pixel_format: pf,
        }
    }

    pub unsafe fn draw_pixel(&self, x: isize, y: isize, color: u32) {
        let fba = self.address as *mut u32;
        let stride = self.stride;
        let offset = x+(y*stride as isize);
        if offset > 0 && offset < self.size as isize {
            fba.wrapping_add(offset.try_into().unwrap()).write_volatile(color);
        }
    }

    pub unsafe fn draw_pixel_col(&self, x: isize, y: isize, color: Color) {
        let col = transmute(color);
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
}
