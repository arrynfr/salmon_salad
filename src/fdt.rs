use core::slice;

const FDT_BEGIN_NODE: u32 = 0x00000001_u32.to_be();
const FDT_END_NODE: u32 =  0x00000002_u32.to_be();
const FDT_PROP: u32 = 0x00000003_u32.to_be();
const FDT_NOP: u32 = 0x00000004_u32.to_be();
const FDT_END: u32 = 0x00000009_u32.to_be();

#[repr(C)]
#[derive(Debug)]
pub struct FdtHeader {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32
}

#[derive(Debug)]
pub struct Fdt {
    header: FdtHeader,
}

pub struct FdtReserveEntry {
    address: u64,
    size: u64
}

#[derive(Debug)]
enum FdtError {
    InvalidMagic,
    NotFound,
}

impl Fdt {
    pub fn new(fdt_addr: *const u8) -> Option<Self> {
        let fdt;
        let hdr = Fdt::_parse_header(fdt_addr).expect("Couldn't parse FDT header");
        let mem_reserve;
        unsafe {
            mem_reserve = slice::from_raw_parts(fdt_addr
                                                    .add(hdr.off_mem_rsvmap as usize) as *const FdtReserveEntry,
                                                    hdr.totalsize as usize);
        }
        for x in mem_reserve {
            if x.address == 0 && x.size == 0 { break; }
            println!("Addr: {} Size: {}", x.address, x.size);
        }
        fdt = Fdt { header: hdr };
        return Some(fdt);
    }

    fn _parse_header(fdt_addr: *const u8) -> Result<FdtHeader, FdtError> {
        let fdt_hdr;
        unsafe {
            fdt_hdr = slice::from_raw_parts(fdt_addr as *const FdtHeader, 1);
        }
        if let Some(fdt_hdr) = fdt_hdr.first() {
            match fdt_hdr.magic.to_be() {
                0xd00dfeed => {
                    let hdr = FdtHeader {
                        magic: fdt_hdr.magic.to_be(),
                        totalsize: fdt_hdr.totalsize.to_be(),
                        off_dt_struct: fdt_hdr.off_dt_struct.to_be(),
                        off_dt_strings: fdt_hdr.off_dt_strings.to_be(),
                        off_mem_rsvmap: fdt_hdr.off_mem_rsvmap.to_be(),
                        version: fdt_hdr.version.to_be(),
                        last_comp_version: fdt_hdr.last_comp_version.to_be(),
                        boot_cpuid_phys: fdt_hdr.boot_cpuid_phys.to_be(),
                        size_dt_strings: fdt_hdr.size_dt_strings.to_be(),
                        size_dt_struct: fdt_hdr.size_dt_struct.to_be()
                    };
                    return Ok(hdr);
                }
                _ => { return Err(FdtError::InvalidMagic); }
            }
        }
        Err(FdtError::NotFound)
    }
}