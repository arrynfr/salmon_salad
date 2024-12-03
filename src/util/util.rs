pub fn hex_print(addr: *mut u8, lines: usize) {
    let num_vals = lines*16;
    unsafe {
        println!("{0:<12}: {1:2X?}", "Offset",[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
        for x in (0..num_vals).step_by(16) {
            let mut values: [u8; 16] = [0; 16];
            for y in 0..16 {
                values[y] = addr.add(x+y).read_volatile();
            }
            println!("{0:<12p}: {1:2x?}", addr.add(x), values);
        }
    }
}

pub fn hex_print32(addr: *mut u8, lines: usize) {
    let num_vals = lines*16;
    unsafe {
        println!("{0:<12}: {1:2X?}", "Offset",[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
        for x in (0..num_vals).step_by(16) {
            let mut values: [u8; 16] = [0; 16];
            for y in (0..16).step_by(4) {
                let val = (addr.add(x+y) as *mut u32).read_volatile().to_le() as u64;
                values[y+0] = ((val >> 0) & 0xFF) as u8;
                values[y+1] = ((val >> 8) & 0xFF) as u8;
                values[y+2] = ((val >> 16) & 0xFF) as u8;
                values[y+3] = ((val >> 32) & 0xFF) as u8;
            }
            println!("{0:<12p}: {1:2x?}", addr.add(x), values);
        }
        println!();
    }
}