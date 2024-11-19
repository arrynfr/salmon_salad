#[repr(C)]
pub struct VirtIOPCICap {
    cap_vndr:   u8,
    cap_next:   u8,
    cap_len:    u8,
    cfg_type:   u8,
    bar:        u8,
    id:         u8,
    padding:    [u8; 2],
    offset:     u32,
    length:     u32
}