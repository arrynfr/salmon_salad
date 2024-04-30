//GICD = 0x8000000 0x10000
//GICR = 0x80a0000 0xf60000
//ITS = 0x8080000 0x20000

use core::{arch::asm, mem::size_of};

const GICD_BASE: *mut u32 = 0x8000000 as *mut u32; // 0x00000001
const GICD_ISENABLER: usize = 0x0100; // 0x6c00ffff
const _GICD_ICENABLER: usize = 0x0180; // 0x6c00ffff
const _GICD_PENDR: usize = 0x0280; // 0x08000000
const GICD_CTLR_EN_GRP0: u32 = 1 << 0;
const GICD_CTLR_EN_GRP1: u32 = 1 << 1;
const _GICD_CTLR_EN_GRP1_S: u32 = 1 << 2;
const GICD_CTLR_ARE: u32 = 1 << 4;
const _GICD_CTLR_DS: u32 = 1 << 6;
const _GICD_CTLR_E1NWF: u32 = 1 << 7;
const _GICD_CTLR_RWP: u32 = 1 << 31;
const SGI_OFFSET: usize = 64*1024; //12.10 The GIC Redistributor register map 64k pages per frame

const GICR_BASE:  *mut u32 = 0x80a0000 as *mut u32;
const _GICR_CTLR: usize = 0x0000;
const _GICR_IIDR: usize = 0x0004;
const _GICR_TYPER: usize = 0x0008;
const _GICR_STATUSR: usize = 0x0010;
const GICR_WAKER: usize = 0x0014;
const GICR_WAKER_PROCESSORSLEEP: u32 = 1 << 1;
const GICR_WAKER_CHILDRENASLEEP: u32 = 1 << 2;
const _GICR_MPAMIDR: usize = 0x0018;
const _GICR_PARTIDR: usize = 0x001C;
//0x20 - 0x3C Implementation Defined
const _GICR_SETLPIR_LO: usize = 0x0040;
const _GICR_SETLPIR_HI: usize = 0x0044;
const _GICR_CLRLPR_LO: usize = 0x0048;
const _GICR_CLRLPR_HI: usize = 0x004C;
//0x50 - 0x6C Reserved
const _GICR_PROPBASER_LO: usize = 0x0070;
const _GICR_PROPBASER_HI: usize = 0x0074;
const _GICR_PENDBASER_LO: usize = 0x0078;
const _GICR_PENDBASER_HI: usize = 0x007C;
const _GICR_INVLPIR_LO: usize = 0x00A0;
const _GICR_INVLPIR_HU: usize = 0x00A4;
const _GICR_INVALLR_LO: usize = 0x00B0;
const _GICR_INVALLR_HI: usize = 0x00B4;
const _GICR_SYNCR: usize = 0x00C0;

const _GICR_IPRIORITYR: usize = 0x0400;
const GICR_IGROUPR0: usize = 0x0080;
const GICR_ISENABLER0: usize = 0x0100;
const GICR_ICFGR :usize = 0x0C00;

const _ITS_BASE:  *mut u32 = 0x8080000 as *mut u32;

pub unsafe fn init_gicd() {
    GICD_BASE.write_volatile(GICD_CTLR_ARE);
    GICD_BASE.write_volatile(GICD_CTLR_EN_GRP1 | GICD_CTLR_EN_GRP0);
    for x in 0..32 {
        GICD_BASE.add(GICD_ISENABLER/size_of::<u32>()+x).write_volatile(0xffffffff);
    }
}

pub unsafe fn init_gicr() {
    let mut gicr_waker = GICR_BASE.add(GICR_WAKER/size_of::<u32>()).read_volatile();
    gicr_waker &= !GICR_WAKER_PROCESSORSLEEP;
    GICR_BASE.add(GICR_WAKER/size_of::<u32>()).write_volatile(gicr_waker);
    while GICR_BASE.add(GICR_WAKER/size_of::<u32>()).read_volatile() & GICR_WAKER_CHILDRENASLEEP != 0 {}
}

pub unsafe fn per_core_init() {
    init_gicr(); // Important to mark PE online before configuring CPU interface
    asm!(   "msr ICC_SRE_EL1, {0:x}",
            "msr ICC_CTLR_EL1, {4:x}",
            "msr ICC_BPR0_EL1, {3:x}",
            "msr ICC_BPR1_EL1, {1:x}",
            "msr ICC_PMR_EL1, {2:x}",
            "msr ICC_IGRPEN1_EL1, {0:x}",
            "msr ICC_IGRPEN0_EL1, {0:x}",
            "/* {1:x} */",
            "isb",
            in(reg) 0b1,
            in(reg) 0b0,
            in(reg) 0xff,
            in(reg) 7,
            in(reg) 0b10,
            options(nostack, nomem)
        );
    GICR_BASE.add((SGI_OFFSET+GICR_ICFGR)/size_of::<u32>()).write_volatile(0);
    GICR_BASE.add((SGI_OFFSET+GICR_IGROUPR0)/size_of::<u32>()).write_volatile(0xffffffff);
    GICR_BASE.add((SGI_OFFSET+GICR_ISENABLER0)/size_of::<u32>()).write_volatile(0xffffffff);
}

pub unsafe fn init_gic() {
    init_gicd();
    per_core_init();
}

pub fn acknowledge_interrupt(intid: u64) {
    unsafe { asm!("msr ICC_EOIR1_EL1, {:x}", in(reg) intid) };
    unsafe { asm!("msr ICC_DIR_EL1, {:x}", in(reg) intid) };
}

pub fn send_sgi(sgiid: u64) {
    assert!(sgiid <= 0xF);
    let sgireg: u64 = 1_u64 << 40 | (sgiid & 0x0f) << 24;
    unsafe {
        asm!(   "msr ICC_SGI1R_EL1, {:x}",
        in(reg) sgireg);
    }
}
// ICC_CTLR_EL1 -> Interrupt Controller Control Register
// GICD_CTLR -> Distributor control register
// Set GICD_CTLR.ARE -> 1
// GICD_IROUTER<n> to select core for SPI