#![no_std]

#[derive(Debug)]
pub struct PPM<'a>{
    pub magic: [u8;2],
    pub width: u16,
    pub heigth: u16,
    pub max_col: u16,
    pub data: &'a[u8]
}

#[derive(Debug)]
pub enum PPMError {
    MalformedHeader,
    MalformedMagic,
    InvalidWidth,
    InvalidHeigth,
    InvalidMaxCol
}

impl<'a> PPM<'a> {
    pub fn new(img_sl: &'a [u8]) -> Result<Self, PPMError>  {
        let mut data_start = 0;
        let mut fields_seen = 0;
        for (i,x) in img_sl.iter().enumerate() {
            if x.is_ascii_whitespace() {
                fields_seen += 1;
                if fields_seen == 4 {
                    data_start = i;
                    break;
                }
            } else if i > 20 { return Err(PPMError::MalformedHeader) }
        }

        let hdr = match core::str::from_utf8(&img_sl[0..data_start]) {
            Ok(hdr) => { hdr },
            Err(_) => { return Err(PPMError::MalformedHeader) }
        };
        let mut hdr_it = hdr.split_whitespace();
        
        let magic = hdr_it.next().ok_or(PPMError::MalformedHeader)?;
        if magic != "P6" { return Err(PPMError::MalformedMagic) }

        let width = match u16::from_str_radix(hdr_it.next().ok_or(PPMError::MalformedHeader)?, 10) {
            Ok(w) => { 
                if w <= u16::max_value() { w }
                else { return Err(PPMError::InvalidWidth) }
             }
            Err(_) => { return Err(PPMError::InvalidWidth) }
        };

        let heigth =  match u16::from_str_radix(hdr_it.next().ok_or(PPMError::MalformedHeader)?, 10) {
            Ok(h) => {
                if h <= u16::max_value() { h }
                else { return Err(PPMError::InvalidHeigth) }
            }
            Err(_) => { return Err(PPMError::InvalidHeigth) }
        };

        let max_col = match u16::from_str_radix(hdr_it.next().ok_or(PPMError::MalformedHeader)?, 10) {
            Ok(m) => { 
                if m <= u16::max_value() { m }
                else { return Err(PPMError::InvalidMaxCol) }
             }
            Err(_) => { return Err(PPMError::InvalidMaxCol) }
        };

        Ok(PPM {
            magic: *b"P6",
            width: width,
            heigth: heigth,
            max_col: max_col,
            data: &img_sl[hdr_it.count()..]
        })
    }
}
