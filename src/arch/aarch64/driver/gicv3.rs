//GICD = 0x8000000 0x10000
//GICR = 0x80a0000 0xf60000
//ITS = 0x8080000 0x20000

use core::{arch::asm, mem::size_of, ptr::{addr_of, addr_of_mut}};

#[repr(C)]
struct Ipriority {
    byte0: u8,
    byte1: u8,
    byte2: u8,
    byte3: u8
}

#[repr(C)]
struct GICD {
    gicd_ctlr:      u32,                //0x0000
    gicd_typer:     u32,                //0x0004
    gicd_iidr:      u32,                //0x0008
    gicd_typer2:    u32,                //0x000C
    gicd_statusr:   u32,                //0x0010
    _reserved0:     [u32; 3],           //0x0014-0x001C
    _imp_def0:      [u32; 8],           //0x0020-0x003C
    gicd_setspi_nsr:u32,                //0x0040
    _reserved1:     u32,                //0x0044
    gicd_clrspi_nsr:u32,                //0x0048
    _reserved2:     u32,                //0x004c
    gicd_setspi_sr: u32,                //0x0050
    _reserved3:     u32,                //0x0054
    gicd_clrspi_sr: u32,                //0x0058
    _reserved4:     [u32; 9],           //0x005c-0x007c
    gicd_igroupr:   [u32; 32],          //0x0080-0x00FC
    gicd_isenabler: [u32; 32],          //0x0100-0x017C
    gicd_icenabler: [u32; 32],          //0x0180-0x01FC
    gicd_ispendr:   [u32; 32],          //0x0200-0x027C
    gicd_icpendr:   [u32; 32],          //0x0280-0x02FC
    gicd_isactiver: [u32; 32],          //0x0300-0x037C
    gicd_icactiver: [u32; 32],          //0x0380-0x03FC
    gicd_ipriorityr:[u32; 256],         //0x0400-0x07F8
    gicd_itargetsr: [u32; 256],         //0x0800-0x0BF8
    gicd_icfgr:     [u32; 64],          //0x0C00-0x0CFC
    gicd_igrpmodr:  [u32; 32],          //0x0D00-0x0D7C
    _unknown0:      [u32; 32],          //0x0D80-0x0DFC
    gicd_nsacr:     [u32; 64],          //0x0E00-0x0EFC
    gicd_sgir:      u32,                //0x0F00
    _unknown1:      [u32;3],            
    gicd_cpendsgir: [u32; 4],           //0x0F10-0x0F1C
    gicd_spendsgir: [u32; 4],           //0x0F20-0x0F2C
    _reserved6:     [u32; 20],          //0x0F30-0x0F7C
    gicd_inmir:     [u32; 32],          //0x0F80-0x0FFC
    gicd_igroupr_e: [u32; 32],          //0x1000-0x107C
    _unknown2:      [u32; 96],
    gicd_isenabler_e:[u32; 32],         //0x1200-0x127C
    _unknown3:      [u32; 96],
    gicd_icenabler_e:[u32; 32],         //0x1400-0x147C
    _unknown4:      [u32; 96],
    gicd_ispendr_e: [u32; 32],          //0x1600-0x167C
    _unknown5:      [u32; 96],
    gicd_icpendr_e: [u32; 32],          //0x1800-0x187C
    _unknown6:      [u32; 96],
    gicd_isactiver_e:[u32; 32],         //0x1A00-0x1A7C 
    _unknown7:      [u32; 96],
    gicd_icactiver_e:[u32; 32],         //0x1C00-0x1C7C
    _unknown8:      [u32; 224],
    gicd_ipriorityr_e:[u32; 256],       //0x2000-0x23FC
    _unknown9:      [u32; 768],
    gicd_icfgr_e:   [u32; 64],          //0x3000-0x30FC 
    _unknown10:     [u32; 192],
    gicd_igrpmodr_e:[u32; 32],          //0x3400-0x347C
    _unknown11:     [u32; 96],
    gicd_nsacr_e:   [u32; 64],          //0x3600-0x36FC
    _reserved7:     [u32; 256],         //0x3700-0x3AFC
    gicd_inmir_e:   [u32; 32],          //0x3B00-0x3B7C
    _reserved8:     [u32; 2400],        //0x3B80-0x60FC
    gicd_irouter:   [u64; 988],         //0x6100-0x7FD8
    _unknown12:     [u32; 8],               
    gicd_irouter_e: [u64; 1024],        //0x8000-0x9FFC
    _reserved9:     [u32; 2048],        //0xA000-0xBFFC
    _imp_def1:      [u32; 4084],        //0xC000-0xFFCC
    id_regs:        [u32; 12]           //0xFFD0-0xFFFC
}

const GICD_BASE: *mut GICD = 0x8000000 as *mut GICD; // 0x00000001
const GICD_ISENABLER: usize = 0x0100; // 0x6c00ffff
const _GICD_ICENABLER: usize = 0x0180; // 0x6c00ffff
const _GICD_PENDR: usize = 0x0280; // 0x08000000
const GICD_CTLR_EN_GRP0: u32 = 1 << 0;
const GICD_CTLR_EN_GRP1: u32 = 1 << 1;
const _GICD_CTLR_EN_GRP1_S: u32 = 1 << 2;
const GICD_CTLR_ARE: u32 = 1 << 4;
const GICD_CTLR_ARE_NS: u32 = 1 << 5;
const _GICD_CTLR_DS: u32 = 1 << 6;
const GICD_CTLR_E1NWF: u32 = 1 << 7;
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
    addr_of_mut!((*GICD_BASE).gicd_ctlr).write_volatile(GICD_CTLR_ARE | GICD_CTLR_ARE_NS);
    let ctlr = addr_of_mut!((*GICD_BASE).gicd_ctlr).read_volatile();
    addr_of_mut!((*GICD_BASE).gicd_ctlr).write_volatile(ctlr | GICD_CTLR_EN_GRP1 | GICD_CTLR_EN_GRP0 | GICD_CTLR_E1NWF);
    for x in 0..32 {
        addr_of_mut!((*GICD_BASE).gicd_isenabler[x]).write_volatile(0xffffffff);
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
            "isb",
            "dsb sy",
            in(reg) 0b1,
            in(reg) 0b0,
            in(reg) 0xff,
            in(reg) 7,
            in(reg) 0b10,
            options(nostack, nomem)
        );
    GICR_BASE.add((SGI_OFFSET+GICR_ICFGR)/size_of::<u32>()).write_volatile(0);
    GICR_BASE.add((SGI_OFFSET+GICR_ISENABLER0)/size_of::<u32>()).write_volatile(0xffffffff);
    GICR_BASE.add((SGI_OFFSET+GICR_IGROUPR0)/size_of::<u32>()).write_volatile(0xffffffff);


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
        asm!(   "msr ICC_SGI0R_EL1, {:x}",
        in(reg) sgireg);
        asm!(   "msr ICC_SGI1R_EL1, {:x}",
        in(reg) sgireg);
    }
}
// ICC_CTLR_EL1 -> Interrupt Controller Control Register
// GICD_CTLR -> Distributor control register
// Set GICD_CTLR.ARE -> 1
// GICD_IROUTER<n> to select core for SPI