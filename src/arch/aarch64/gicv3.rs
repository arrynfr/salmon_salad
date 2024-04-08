//GICD = 0x8000000 0x10000
//GICR = 0x80a0000 0xf60000
//ITS = 0x8080000 0x20000

const GICD_BASE: *mut u32 = 0x8000000 as *mut u32;
const GICD_CTLR_EN_GRP0: u32 = 1 << 0;
const GICD_CTLR_EN_GRP1: u32 = 1 << 1;
const _GICD_CTLR_EN_GRP1_S: u32 = 1 << 2;
const GICD_CTLR_ARE: u32 = 1 << 4;
const GICD_CTLR_DS: u32 = 1 << 6;
const _GICD_CTLR_E1NWF: u32 = 1 << 7;
const _GICD_CTLR_RWP: u32 = 1 << 31;

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

const _ITS_BASE:  *mut u32 = 0x8080000 as *mut u32;

pub unsafe fn init_gicd() {
    let mut gicd_ctlr: u32 = GICD_BASE.read_volatile();
    gicd_ctlr &= !(GICD_CTLR_EN_GRP0 & GICD_CTLR_EN_GRP1);
    GICD_BASE.write_volatile(gicd_ctlr);

    if gicd_ctlr & (GICD_CTLR_EN_GRP0 | GICD_CTLR_EN_GRP1) == 0 {
        println!("Interrupts are not enabled!");
    } else {
        println!("Interrupts are enabled!");
    }
    gicd_ctlr |= GICD_CTLR_ARE;
    GICD_BASE.write_volatile(gicd_ctlr);
    gicd_ctlr = GICD_BASE.read_volatile();
    println!("GICD_CTLR: {:x?}", gicd_ctlr&GICD_CTLR_DS);
}

pub unsafe fn init_gicr() {
    let mut gicr_waker = GICR_BASE.add(GICR_WAKER).read_volatile();
    gicr_waker &= !GICR_WAKER_PROCESSORSLEEP;
    GICD_BASE.add(GICR_WAKER).write_volatile(gicr_waker);
    while GICR_BASE.add(GICR_WAKER).read_volatile() & GICR_WAKER_CHILDRENASLEEP != 0 {
        println!("Waiting on childrenasleep");
    }
}

pub unsafe fn init_gic() {
    init_gicd();
    init_gicr();
}
// ICC_CTLR_EL1 -> Interrupt Controller Control Register
// GICD_CTLR -> Distributor control register

// Set GICD_CTLR.ARE -> 1