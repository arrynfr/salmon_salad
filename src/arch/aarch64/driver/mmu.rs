use core::arch::asm;

const L1TABLE_SIZE: usize = 64;
const L2TABLE_SIZE: usize = 8192;

#[repr(C, align(65536))]
struct TranslationTable {
    pub table: [[u64; L2TABLE_SIZE]; L1TABLE_SIZE]
}

#[repr(C, align(65536))]
struct L1TranslationTable {
    table: [u64; L1TABLE_SIZE]
}

static mut L1TABLES: L1TranslationTable = L1TranslationTable {
    table: [0; L1TABLE_SIZE]
};

static mut L2TABLES: TranslationTable = TranslationTable {
    table: [[0; L2TABLE_SIZE]; L1TABLE_SIZE]
};


const MAIR_DEVICE_N_GN_RN_E: u64 =  0b00000000;
const MAIR_NORMAL_NOCACHE: u64 =  0b01000100;
const MAIR_IDX_DEVICE_N_GN_RN_E: u64 =  0;
const MAIR_IDX_NORMAL_NOCACHE: u64 =  1;

const TCR_CONFIG_REGION_48BIT: u64 =  ((64 - 48) << 0) | ((64 - 48) << 16) | 0b101 << 32;
const TCR_CONFIG_64KB:u64 = (0b01 << 14) |  (0b11 << 30);
const TCR_CONFIG_DEFAULT: u64 =  TCR_CONFIG_REGION_48BIT | TCR_CONFIG_64KB;

const PD_ACCESS: u64 = 1 << 10;
const BOOT_PUD_ATTR: u64 =  PD_ACCESS | (MAIR_DEVICE_N_GN_RN_E << 2);

pub fn mmu_test() {
    unsafe {
        println!("Enabling MMU");
        init_identity_mapping();
        println!("MMU enabled");
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

unsafe fn init_identity_mapping() {
    let mair_el1 =  (MAIR_DEVICE_N_GN_RN_E << (MAIR_IDX_DEVICE_N_GN_RN_E << 3)) | //ATTR0 NGNRNE
    (MAIR_NORMAL_NOCACHE << (MAIR_IDX_NORMAL_NOCACHE << 3)); //ATTR1 Normal no cache

    let pgd_addr = L1TABLES.table.as_ptr() as u64;
    for x in 0..L1TABLES.table.len() {
        L1TABLES.table[x] = create_table_descriptor(L2TABLES.table[x].as_ptr() as u64);
        for y in 0..L2TABLES.table[x].len() {
            let maxi = x*8191*0x20000000;
            L2TABLES.table[x][y] = create_block_entry((maxi+y*0x20000000).try_into().unwrap());
        }
    }

    asm!("msr TTBR0_EL1, {:x}", in(reg) pgd_addr);
    asm!("msr tcr_el1, {:x}", in(reg) TCR_CONFIG_DEFAULT);
    asm!("msr mair_el1, {:x}", in(reg) mair_el1);
    asm!("tlbi vmalle1",
    "dsb ish",
    "isb");

    let mut sctlr_el1: u64;
    asm!("mrs {:x}, SCTLR_EL1", out(reg) sctlr_el1);
    sctlr_el1 = sctlr_el1 | (1 << 0);
    asm!("msr SCTLR_EL1, {:x}", "isb", in(reg) sctlr_el1);
}