//GICD = 0x8000000 0x10000
//GICR = 0x80a0000 0xf60000
//ITS = 0x8080000 0x20000

use core::{arch::asm, ptr::addr_of_mut};

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
    gicd_pidr4:     u32,
    gicd_pidr5:      u32,
    gicd_pidr6:      u32,
    gicd_pidr7:      u32,
    gicd_pidr0:      u32,
    gicd_pidr1:      u32,
    gicd_pidr2:      u32,
    gicd_pidr3:      u32,
    gicd_cidr0:      u32,
    gicd_cidr1:      u32,
    gicd_cidr2:      u32,
    gicd_cidr3:      u32,
}

impl GICD {
    const CTLR_EN_GRP0: u32 = 1 << 0;
    const CTLR_EN_GRP1: u32 = 1 << 1;
    const _CTLR_EN_GRP1_S: u32 = 1 << 2;
    const CTLR_ARE: u32 = 1 << 4;
    const CTLR_ARE_NS: u32 = 1 << 5;
    const _CTLR_DS: u32 = 1 << 6;
    const CTLR_E1NWF: u32 = 1 << 7;
    const _CTLR_RWP: u32 = 1 << 31;
}

struct GICR {
    rd_base:    *mut RDBase,
    sgi_base:   *mut SGIBase,
}

impl GICR {
    const WAKER_PROCESSORSLEEP: u32 = 1 << 1;
    const WAKER_CHILDRENASLEEP: u32 = 1 << 2;
}

#[repr(C)]
struct RDBase {
    gicr_ctlr:      u32,
    gicr_iidr:      u32,
    gicr_typer:     u32,
    _reserved0:     u32,
    gicr_statusr:   u32,
    gicr_waker:     u32,
    gicr_mpamidr:   u32,
    gicr_partidr:   u32,
    _imp_def0:      [u32; 8],
    gicr_setlpir:   u64,
    gicr_clrlpir:   u64,
    _reserved1:      [u32; 8],
    gicr_propbaser: u64,
    gicr_pendbaser: u64,
    _unknown0:       [u32; 8],
    gicr_invlpir:   u64,
    _reserved2:     u64,
    gicr_invallr:   u64,
    _reserved3:     u64,
    gicr_syncr:     u64,
    _reserved4:     [u32; 14],
    _imp_def1:      u64,
    _reserved5:     u64,
    _imp_def2:      u64,
    _reserved6:     [u32; 12218],
    _imp_def3:      [u32; 0xFF4],
    _imp_def4:      [u32; 6],
    gicr_pidr2:     u32,
    _imp_def5:      [u32; 5],
}

#[repr(C)]
struct SGIBase {
    _unknown0:          [u32; 32],
    gicr_igroupr0:      u32,
    gicr_igroupr_e:     [u32; 2],
    _unknown1:          [u32; 29],
    gicr_isenabler0:    u32,
    gicr_isenabler_e:   [u32; 2],
    _unknown2:          [u32; 29],
    gicr_icenabler:     u32,
    gicr_icenabler_e:   [u32; 2],
    _unknown3:          [u32; 29],
    gicr_ispendr0:      u32,
    gicr_ispendr_e:     [u32; 2],
    _unknown4:          [u32; 29],
    gicr_icpendr0:      u32,
    gicr_icpendr_e:     [u32; 2],
    _unknown5:          [u32; 29],
    gicr_isactiver0:    u32,
    gicr_isactiver_e:   [u32; 2],
    _unknown6:          [u32; 29],
    gicr_icactiver0:    u32,
    gicr_icactiver_e:   [u32; 2],
    _unknown7:          [u32; 29],
    gicr_ipriorityr:    [u32; 8],
    gicr_ipriorityr_e:  [u32; 16],
    _unknown8:          [u32; 488],
    gicr_icfgr0:        u32,
    gicr_icfgr1:        u32,
    gicr_icfgr_e:       [u32; 4],
    _unknown9:          [u32; 58],
    gicr_igrpmodr0:     u32,
    gicr_igrpmodr_e:    [u32; 2],
    _unknown10:         [u32; 61],
    gicr_nsacr:         u32,
    _reserved0:         [u32; 95],
    gicr_inmir0:        u32,
    gicr_inmir_e:       [u32; 31],
    _reserved1:         [u32; 11264],
    _imp_def0:          [u32; 4084],
    _reserved2:         [u32; 12]
}

pub struct GIC {
    gicd: *mut GICD,
    gicr: GICR
}

#[derive(Debug)]
pub enum GICError {
    UnsupportedVersion,
    InvalidAddress,
    NoRedistributorFound
}

impl GIC {
    const FRAME_SIZE: usize = 64*1024;
    const NUM_FRAMES: usize = 2;
    const REDISTRIBUTOR_SIZE: usize = GIC::FRAME_SIZE*GIC::NUM_FRAMES;
    const RD_BASE: usize = GIC::FRAME_SIZE*0;
    const SGI_BASE: usize = GIC::FRAME_SIZE*1;

    const MAX_PPI: u64 = 31;
    
    pub unsafe fn new(gicd_base: *mut u8, gicr_base: *mut u8) -> Result<Self, GICError> {
        if gicd_base == core::ptr::null_mut() || gicr_base == core::ptr::null_mut() {
            return Err(GICError::InvalidAddress)
        }

        let version = (*(gicd_base as *mut GICD)).gicd_pidr2 >> 4;
        if version == 3 {

            //Find redistributor for the current core
            let mut gicr: Option<GICR> = None;
            let current_core = _get_current_core();
            for x in 0..4 { //TODO: Get num cores from device tree
                let gicr_typer = (*((gicr_base.add(x*GIC::FRAME_SIZE*2)) as *mut RDBase)).gicr_typer;
                let rd_cpu_number = (gicr_typer & (0xFFFF << 8)) >> 8;
                if rd_cpu_number == current_core as u32 {
                    gicr= Some(GICR {
                        rd_base: gicr_base.add(x*GIC::REDISTRIBUTOR_SIZE+GIC::RD_BASE) as *mut RDBase,
                        sgi_base: gicr_base.add(x*GIC::REDISTRIBUTOR_SIZE+GIC::SGI_BASE) as *mut SGIBase,
                    })
                }
            }

            if let Some(gicr) = gicr {
                return Ok
                (GIC {
                    gicd: gicd_base as *mut GICD,
                    gicr: gicr
                })
            } 
            return Err(GICError::NoRedistributorFound)
        }

        Err(GICError::UnsupportedVersion)
    }

    pub unsafe fn init_gicd(&self) {
        addr_of_mut!((*self.gicd).gicd_ctlr).write_volatile(GICD::CTLR_ARE | GICD::CTLR_ARE_NS);
        let ctlr = addr_of_mut!((*self.gicd).gicd_ctlr).read_volatile();
        addr_of_mut!((*self.gicd).gicd_ctlr).write_volatile(ctlr | GICD::CTLR_EN_GRP1 | GICD::CTLR_EN_GRP0 | GICD::CTLR_E1NWF);
    }

    pub unsafe fn init_gicr(&self) {
        let mut gicr_waker = addr_of_mut!((*self.gicr.rd_base).gicr_waker).read_volatile();
        gicr_waker &= !GICR::WAKER_PROCESSORSLEEP;
        addr_of_mut!((*self.gicr.rd_base).gicr_waker).write_volatile(gicr_waker);
        while addr_of_mut!((*self.gicr.rd_base).gicr_waker).read_volatile() & GICR::WAKER_CHILDRENASLEEP != 0 {}
    }

    pub unsafe fn per_core_init(&self) {
        self.init_gicr(); // Important to mark PE online before configuring CPU interface
        asm!(   "msr ICC_SRE_EL1, {0:x}",
                "msr ICC_CTLR_EL1, {1:x}",
                "msr ICC_BPR0_EL1, {2:x}",
                "msr ICC_BPR1_EL1, {3:x}",
                "msr ICC_PMR_EL1, {4:x}",
                "msr ICC_IGRPEN1_EL1, {5:x}",
                "msr ICC_IGRPEN0_EL1, {5:x}",
                "isb",
                "dsb sy",
                in(reg) 0b111,
                in(reg) 0b10,
                in(reg) 7,
                in(reg) 0b0,
                in(reg) 0xff,
                in(reg) 0b1,
                options(nostack, nomem)
            );
    }

    pub unsafe fn init_gic(&self) {
            self.init_gicd();
            self.per_core_init();
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

    pub fn enable_interrupt(&self, intid: u64) {
        assert!(intid <= 1024);
        unsafe {
            if intid <= GIC::MAX_PPI {
                addr_of_mut!((*self.gicr.sgi_base).gicr_isenabler0).write_volatile(1 << intid);
            } else {
                let reg_num: usize = (intid/32) as usize;
                let enable_bit = intid % 32;
                addr_of_mut!((*self.gicd).gicd_isenabler[reg_num]).write_volatile(1 << enable_bit);
            }
        }
    }

    pub fn set_interrupt_group(&self, intid: u64, group0: bool) {
        assert!(intid <= 1024);
        let group0_value = group0 as u32;
        unsafe {
            if intid <= GIC::MAX_PPI {
                addr_of_mut!((*self.gicr.sgi_base).gicr_igroupr0).write_volatile(group0_value << intid);
            } else {
                let reg_num: usize = (intid/32) as usize;
                let enable_bit = intid % 32;
                addr_of_mut!((*self.gicd).gicd_igroupr[reg_num]).write_volatile(1 << enable_bit);
            }
        }
    }

    pub fn set_interrupt_trigger(&self, intid: u64, edge_triggered: bool) {
        assert!(intid <= 1024);
        assert!(intid >= 16); //SGIs are always edge triggered
        let trigger = if edge_triggered == true { 0b10 } else { 0b00 };
        unsafe {
            if intid <= GIC::MAX_PPI {
                let enable_bit = (intid%16)*2;
                addr_of_mut!((*self.gicr.sgi_base).gicr_icfgr1).write_volatile(trigger << enable_bit);
            } else {
                let reg_num = (intid/16) as usize;
                let enable_bit = (intid%16)*2;
                addr_of_mut!((*self.gicd).gicd_icfgr[reg_num]).write_volatile(trigger << enable_bit);
            }
        }
    }
}

fn _get_current_core() -> u64 {
    let current_core: u64;
    unsafe { asm!("mrs {}, MPIDR_EL1",
                out(reg) current_core,
                options(nostack, nomem));
            }
    current_core&0xFF
}

pub const GICD_BASE: *mut u8 = 0x8000000 as *mut u8; // 0x00000001
pub const GICR_BASE:  *mut u8 = 0x80a0000 as *mut u8;