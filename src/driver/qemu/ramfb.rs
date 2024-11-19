use core::ffi::CStr;
use core::ptr::addr_of;
use core::mem;

use crate::arch::host::driver::mmu::va_to_pa;

#[repr(C)]
#[derive(Debug)]
struct FWCfgFile {
    size: u32,
    select: u16,
    reserved: u16,
    name: [u8; 56]
}

#[repr(C, packed)]
#[derive(Debug)]
struct FWCfgDmaAccess {
    control: u32,
    len: u32,
    addr: u64
}

#[repr(C, packed)]
#[derive(Debug)]
struct RamFBCfg {
    addr: u64,
    fmt: u32,
    flags: u32,
    w: u32,
    h: u32,
    st: u32
}

const QEMU_CFG_DMA_CTL_ERROR: u32 = 0x01;
const QEMU_CFG_DMA_CTL_READ: u32 =  0x02;
const _QEMU_CFG_DMA_CTL_SKIP: u32 =  0x04;
const QEMU_CFG_DMA_CTL_SELECT: u32 =0x08;
const QEMU_CFG_DMA_CTL_WRITE: u32 = 0x10;

unsafe fn qemu_dma_transfer (control: u32, len: u32, addr: u64) {
    #[cfg(target_arch = "riscv64")]
    let fw_cfg_dma: *mut u64 = 0x10100010 as *mut u64;

    #[cfg(target_arch = "aarch64")]
    let fw_cfg_dma: *mut u64 = 0x9020010 as *mut u64;
    
    let dma = FWCfgDmaAccess {
        control: control.to_be(),
        len: len.to_be(),
        addr: addr.to_be()
    };
    let dma_addr = va_to_pa(addr_of!(dma) as usize).unwrap() as u64;
    unsafe {
        fw_cfg_dma.write_volatile(dma_addr.to_be());
    }

    while (dma.control & !QEMU_CFG_DMA_CTL_ERROR) != 0 {}
    if (dma.control & QEMU_CFG_DMA_CTL_ERROR) == 1 {
        println!("An error occured in qemu_dma_transfer");
    }
    

}

pub fn setup_ramfb(fb_addr: *mut u8, width: u32, height: u32) {
    let mut num_entries: u32 = 0xFFFFFFFF;
    unsafe {
        qemu_dma_transfer(0x19 << 16| QEMU_CFG_DMA_CTL_SELECT | QEMU_CFG_DMA_CTL_READ,
                        4,  va_to_pa(addr_of!(num_entries) as usize).unwrap() as u64);
    }
    num_entries = num_entries.to_be();

    let ramfb = FWCfgFile {
        size: 0,
        select: 0,
        reserved: 0,
        name: [0; 56]
    };

    for _ in 0..num_entries {
        unsafe {
            qemu_dma_transfer(QEMU_CFG_DMA_CTL_READ, mem::size_of::<FWCfgFile>() as u32,
            va_to_pa(addr_of!(ramfb) as usize).unwrap() as u64);
        }
        let entry = CStr::from_bytes_until_nul(&ramfb.name).unwrap();
        let entry = entry.to_str().unwrap();
        if entry == "etc/ramfb" {
            break;
        }
    }
    //println!("{:#x?}",ramfb.select.to_be());
    let pixel_format = ('R' as u32) | (('G' as u32) << 8) | 
    (('2' as u32) << 16) | (('4' as u32) << 24);
        
    //println!("Placing fb at: {fb_addr:#x?}");
    let bpp: i32 = 3;
    let ramfb_cfg = RamFBCfg {
        addr: (fb_addr as u64).to_be(),
        fmt: (pixel_format).to_be(),
        flags: 0_u32.to_be(),
        w: width.to_be(),
        h: height.to_be(),
        st: (width*bpp as u32).to_be()
    };

    unsafe {
        qemu_dma_transfer((ramfb.select.to_be() as u32) << 16 
    |   QEMU_CFG_DMA_CTL_SELECT 
    |   QEMU_CFG_DMA_CTL_WRITE, mem::size_of::<RamFBCfg>() as u32, va_to_pa(addr_of!(ramfb_cfg) as usize).unwrap() as u64);
    }

}
