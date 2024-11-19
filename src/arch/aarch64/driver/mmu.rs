use core::arch::asm;
use crate::arch::aarch64::platform;

const L0TABLE_SIZE: usize = 2;
const L1TABLE_SIZE: usize = 2048; //64
const L2TABLE_SIZE: usize = 2048; //8192

#[repr(C, align(65536))]
struct TranslationTable {
    pub table: [[u64; L2TABLE_SIZE]; L1TABLE_SIZE]
}

#[repr(C, align(65536))]
struct L1TranslationTable {
    table: [[u64; L1TABLE_SIZE]; L0TABLE_SIZE]
}

#[repr(C, align(65536))]
struct L0TranslationTable {
    table: [u64; L0TABLE_SIZE]
}

static mut L0TABLE: L0TranslationTable = L0TranslationTable {
    table: [0; L0TABLE_SIZE]
};

static mut L1TABLE: L1TranslationTable = L1TranslationTable {
    table: [[0; L1TABLE_SIZE]; L0TABLE_SIZE] // 2048*2
};

static mut L2TABLES: TranslationTable = TranslationTable {
    table: [[0; L2TABLE_SIZE]; L1TABLE_SIZE] // 2048*2048
};


const MAIR_DEVICE_N_GN_RN_E: u64 =  0b00000000;
const MAIR_NORMAL_NOCACHE: u64 =  0b01000100;
const MAIR_IDX_DEVICE_N_GN_RN_E: u64 =  0;
const MAIR_IDX_NORMAL_NOCACHE: u64 =  1;

const TCR_CONFIG_REGION_48BIT: u64 =  ((64 - 48) << 0) | ((64 - 48) << 16) | 0b101 << 32;
const TCR_CONFIG_64KB:u64 = (0b01 << 14) |  (0b11 << 30);
const TCR_CONFIG_16KB:u64 = (0b10 << 14) |  (0b01 << 30);
const TCR_CONFIG_DEFAULT: u64 =  TCR_CONFIG_REGION_48BIT | TCR_CONFIG_16KB;

const PD_ACCESS: u64 = 1 << 10;
const BOOT_PUD_ATTR: u64 = PD_ACCESS | (MAIR_DEVICE_N_GN_RN_E << 2);

#[no_mangle]
pub extern fn enable_mmu() {
    unsafe {
        //println!("Enabling MMU");
        let mmu_is_enabled: u64;
        asm!("mrs {:x}, SCTLR_EL1", out(reg) mmu_is_enabled);
        if mmu_is_enabled & 0b1 == 0 {
            init_identity_mapping();
        }
        //println!("MMU enabled");
    }
}

pub fn va_to_pa(va: usize) -> Result<usize, usize> {
    let pa: usize;
    unsafe {
        asm!("AT S1E1R, {:x}",
        "mrs {:x}, PAR_EL1",
        in(reg) va,
        out(reg) pa);
    }
    if pa & 0b1 == 1 {
        Err(pa)
    } else {
        Ok(((pa & !(0xFFF)) & !(0xFFF << 52)) | va & 0xFFF)
    }
}

fn create_block_entry(address: u64)-> u64 {
    const PD_BLOCK: u64 = 0b01;
    (address & 0xFFFFFFFFF000) | PD_BLOCK | BOOT_PUD_ATTR
}

fn create_table_entry(address: u64)-> u64 {
    (address & 0xFFFFFFFFF000) | (0x1 << 10) | 0x1 // Lower attributes: AF, SH, AP, etc.
}

fn create_table_descriptor(table_addr: u64) -> u64 {
    const TABLE_DESCRIPTOR: u64 = 0x3;
    const ADDRESS_MASK: u64 = 0xFFFFFFFFF000; // Mask for table address
    TABLE_DESCRIPTOR | table_addr & ADDRESS_MASK
}

unsafe fn map_memory(phys_addr: u64) {

}

unsafe fn init_identity_mapping() {
    let pgd_addr = L0TABLE.table.as_ptr() as u64;
    let mair_el1 =  (MAIR_DEVICE_N_GN_RN_E << (MAIR_IDX_DEVICE_N_GN_RN_E << 3)) | //ATTR0 NGNRNE
        (MAIR_NORMAL_NOCACHE << (MAIR_IDX_NORMAL_NOCACHE << 3)); //ATTR1 Normal no cache
    if platform::is_boot_core() {
         //println!("Creating page tables...");
        //let start = platform::get_current_poweron_time_in_ms();
        for l0idx in 0..L0TABLE.table.len()               {
            L0TABLE.table[l0idx] = create_table_descriptor(L1TABLE.table[l0idx].as_ptr() as u64);
            for l1idx in 0..L1TABLE.table[l0idx].len() {
                L1TABLE.table[l0idx][l1idx] = create_table_descriptor(L2TABLES.table[l1idx].as_ptr() as u64);
                for l2idx in 0..L2TABLES.table[l1idx].len() {
                    let page_size: usize = 32<<20;
                    let max_val: usize = page_size*2048;
                    let maxi = l1idx*max_val;
                    L2TABLES.table[l1idx][l2idx] = create_block_entry((maxi+l2idx*page_size) as u64);
                }
            }
        }

    //L0TABLE.table[1] = create_block_entry(0x800000000000);  
        L2TABLES.table[0][4] = L2TABLES.table[0][4] | PD_ACCESS;
        L2TABLES.table[0][32] = L2TABLES.table[0][32] | PD_ACCESS;
    //let end = platform::get_current_poweron_time_in_ms();
    //println!("Done creating identity page tables in {}", end-start);

    /* 64K pages
    let pgd_addr = L1TABLE.table.as_ptr() as u64;
    for x in 0..L1TABLE.table.len() {
        L1TABLE.table[x] = create_table_descriptor(L2TABLES.table[x].as_ptr() as u64);
        for y in 0..L2TABLES.table[x].len() {
            let maxi = x*8191*0x20000000;
            L2TABLES.table[x][y] = create_block_entry((maxi+y*0x20000000).try_into().unwrap());
        }
    }*/
    }
    asm!("msr TTBR0_EL1, {:x}", in(reg) pgd_addr);
    asm!("msr TTBR1_EL1, {:x}", in(reg) pgd_addr);

    asm!("msr tcr_el1, {:x}", in(reg) TCR_CONFIG_DEFAULT);
    asm!("msr mair_el1, {:x}", in(reg) mair_el1);
    asm!("tlbi vmalle1",
    "dsb ish",
    "isb");

    let mut sctlr_el1: u64;
    asm!("mrs {:x}, SCTLR_EL1", out(reg) sctlr_el1);
    //sctlr_el1 = (sctlr_el1 & !(1 << 3 | 1 << 4 | 1 << 5 | 1 << 6 | 1 << 9 | 1 << 11 )) | (1 << 0) | (1 << 2) | (1 << 7) | (1 << 8);
    sctlr_el1 = 0x30901185;
    asm!("msr SCTLR_EL1, {:x}", "isb", "dsb ish", in(reg) sctlr_el1);
}
