use core::ptr::addr_of;

const PL031_BASE: *mut Pl031 = 0x9010000 as *mut Pl031;

#[repr(C)]
struct Pl031 {
    rtc_dr:         u32,
    rtc_mr:         u32,
    rtc_lr:         u32,
    rtc_cr:         u32,
    rtc_imsc:       u32,
    rtc_ris:        u32,
    rtc_mis:        u32,
    rtc_icr:        u32,
    reserved0:      [u32; 24],
    reserved1:      [u32; 5],
    reserved2:      [u32; 975],
    reserved3:      [u32; 4],
    rtc_periph_id0: u32,
    rtc_periph_id1: u32,
    rtc_periph_id2: u32,
    rtc_periph_id3: u32,
    rtc_pcell_id0:  u32,
    rtc_pcell_id1:  u32,
    rtc_pcell_id2:  u32,
    rtc_pcell_id3:  u32
}

