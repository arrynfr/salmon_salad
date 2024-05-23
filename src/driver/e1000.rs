use core::{arch::asm, ptr::{addr_of, addr_of_mut}};

use super::pci::PCIDevice;

pub struct e1000 {
    device: *mut PCIDevice,
    io_space: *mut u32,
    memory_space: *mut u32,
    tx_buffer: [[u8;2048]; 32],
    tx_descs: [LegacyTransDesc; 32],
    tx_tail:    u32,

    rx_descs: [LegacyRecvDesc; 32],
    rx_buffer: [[u8;2048]; 32]
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct LegacyRecvDesc {
    buffer: u64,
    len:    u16,
    chksum: u16,
    status: u8,
    errors: u8,
    special:u16
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct LegacyTransDesc {
    buffer:         u64,
    len:            u16,
    chksum_offset:  u8,
    cmd:            u8,
    sta_rsv:        u8,
    css:            u8,
    special:        u16
} //TDESC.DEXT = 0

impl e1000 {
    const REG_CTRL:     usize = 0x0000;
    const REG_STATUS:   usize = 0x0008;
    const REG_EECD:     usize = 0x0010;
    const REG_EERD:     usize = 0x0014;
    const REG_CTRL_EXT: usize = 0x0018;
    const REG_FLA:      usize = 0x001C;
    const REG_MDIC:     usize = 0x0020;
    const REG_ICR:      usize = 0x00C0;
    const REG_IMS:      usize = 0x00D0;
    const REG_MTA:      usize = 0x5200;
    const REG_MTA_END: usize = 0x0200;

    const CTRL_ASDE:    u32 = (1 << 5);
    const CTRL_SLU:     u32 = (1 << 6);
    const CTRL_ILOS:    u32 = (1 << 7);
    const CTRL_VME:     u32 = (1 << 30);

    const IMS_TXDW:     u32 = 1 << 0;
    const IMS_TXQE:     u32 = 1 << 1;
    const IMS_LSC:      u32 = 1 << 2;
    const IMS_RXSEQ:    u32 = 1 << 3;
    const IMS_RXDMT0:   u32 = 1 << 4;
    const IMS_RXO:      u32 = 1 << 6;
    const IMS_RXT0:     u32 = 1 << 7;
    const IMS_MDAC:     u32 = 1 << 9;
    const IMS_RXCFG:    u32 = 1 << 10;
    const IMS_PHYINT:   u32 = 1 << 12;
    const IMS_TXD_LOW:  u32 = 1 << 15;
    const IMS_SRPD:     u32 = 1 << 16;

    pub fn new(dev: *mut PCIDevice, memory_space: usize, io_space: usize) -> Self {
        e1000 {
            device: dev,
            io_space: io_space as *mut u32,
            memory_space: memory_space as *mut u32,
            tx_buffer: [[0; 2048]; 32],
            tx_descs: [LegacyTransDesc::default(); 32],
            tx_tail: 0,
            rx_buffer: [[0; 2048]; 32],
            rx_descs: [LegacyRecvDesc::default(); 32]
        }
    }

    #[no_mangle]
    pub fn init_my_e1000(&mut self) {
        unsafe {
            addr_of_mut!((*self.device).bar1).write_volatile(self.io_space as u32);
            addr_of_mut!((*self.device).bar0).write_volatile(self.memory_space as u32);
            //addr_of_mut!((*self.device).exp_rom_baddr).write_volatile(0x3000_0000 as u32 | 1);
            addr_of_mut!((*self.device).header.command).write_volatile(0b110); // Enable device in PCI-E config
            println!("Remapped e1000...");
            println!("{:#x?}", *self.device);

            self.reset();
            self.write_reg(e1000::REG_CTRL, (e1000::CTRL_ASDE | e1000::CTRL_SLU) 
                                            & !(e1000::CTRL_ILOS | e1000::CTRL_VME));
            let status = self.read_reg(e1000::REG_STATUS);
            println!("Status after reset: {status:x?}");

            for x in (e1000::REG_MTA..e1000::REG_MTA_END).step_by(4) {
                self.write_reg(x, 0);
            }
            self.write_reg(e1000::REG_IMS, e1000::IMS_LSC | e1000::IMS_RXDMT0 | e1000::IMS_RXO | e1000::IMS_RXT0);
            self.init_rx();
            self.init_tx();

            let buf: [u8; 8] = [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
            self.send_packet(&buf);
            
            //println!("{:p} {:#x?}", addr_of!(rx_descs), rx_descs);
            //loop {
            //    println!("Waiting for packets");
            //    while core::ptr::read_volatile(addr_of!(rx_descs[0].status)) == 0 {}
            //    println!("Got packet...");
            //}
            //print!("{:#x?}", rx_descs[0]);
            

        }
    }

    pub fn init_rx(&mut self) {
        for x in 0..self.rx_descs.len() {
            self.rx_descs[x].buffer = self.rx_buffer[x].as_ptr() as u64;
        }
        let hi = (addr_of!(self.rx_descs) as u64 >> 32) as u32;
        let lo = addr_of!(self.rx_descs) as u32;
        let qlen = core::mem::size_of_val(self.rx_descs.as_slice()) as u32;
        println!("{:x}_{:x}: {:x}", hi, lo, qlen);
        self.write_reg(0x2800, lo); //addr lo
        self.write_reg(0x2804, hi); //addr hi
        self.write_reg(0x2808, qlen); //Len in bytes
        self.write_reg(0x2810, 0_u32); //Head
        self.write_reg(0x2818, (self.rx_descs.len()+1) as u32); //Tail
        self.write_reg(0x100,   (1 << 1)| (1 << 2) | (1 << 3) |
                                (1 << 4) | (1 << 15) | (0 << 16));
    }

    pub fn receive_packet() {
        todo!()
    }

    pub fn init_tx(&mut self) {
        for x in 0..self.tx_descs.len() {
            self.tx_descs[x].buffer = 0x0 as u64;
        }
        let hi = (addr_of!(self.tx_descs) as u64 >> 32) as u32;
        let lo = addr_of!(self.tx_descs) as u32;
        self.write_reg(0x3800, lo);
        self.write_reg(0x3804, hi);
        let qlen = core::mem::size_of_val(self.tx_descs.as_slice()) as u32;
        self.write_reg(0x3808, qlen);
        self.write_reg(0x3810, 0);
        self.write_reg(0x3818, self.tx_tail);
        self.write_reg(0x400, (1 << 1) | (1 << 3) | (0x0F << 4) | (0x200 << 12));
        self.write_reg(0x410, (10 << 0) | (10 << 10) | (10 << 20));
    }

    pub fn send_packet(&mut self, buffer: &[u8]) {
        assert!(buffer.len() <= 2048);
        let tail = self.tx_tail as usize;
        let next_tail = ((tail+1)%self.tx_descs.len()) as u32;
        self.tx_descs[tail].buffer = buffer.as_ptr() as u64;
        self.tx_descs[tail].len = buffer.len() as u16;
        self.tx_descs[tail].cmd = 0b0000_1011;
        self.tx_descs[tail].sta_rsv = 0;
        self.write_reg(0x3818, next_tail);
        unsafe {
            while addr_of_mut!(self.tx_descs[tail].sta_rsv).read_volatile() & 0xFF == 0 {}; 
        }
        self.tx_tail = next_tail;
    }

    pub fn write_reg(&mut self, reg: usize, value: u32) {
        assert!(reg%core::mem::size_of::<u32>() == 0); // regs are 4byte aligned
        assert!(reg <= 128*1024); // Address space is 128k size
        unsafe {
            self.memory_space.add(reg/4).write_volatile(value.to_le());
        }
    }

    pub fn read_reg(&self, reg: usize) -> u32 {
        assert!(reg%core::mem::size_of::<u32>() == 0); // regs are 4byte aligned
        assert!(reg <= 128*1024); // Address space is 128k size
        unsafe {
            self.memory_space.add(reg/4).read_volatile()
        }
    }

    pub fn reset(&mut self) {
        self.write_reg(0x0, 1 << 26);
        while self.read_reg(0x0) & (1 << 26) != 0 {};
    }

    pub fn is_eeprom_present(&self) -> bool {
        self.read_reg(0x0014) & 1 << 0x10 != 0
    }
}

/*
Fiber Copper Fiber
(82544GC/EI
Copper
(82544GC/EI
IPGT 10 10 6 8
IPGR1 10 10 8a
a. Applicable to the 82541xx and 82547GI/EI.
8a
IPGR2 10 10 6a 6a

*/