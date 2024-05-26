use core::ptr::addr_of_mut;

const MAX_DEVICES: usize = 4096;

#[derive(Debug)]
#[repr(C)]
pub struct PCIHeader {
    pub vendor_id:      u16,
    pub device_id:      u16,
    pub command:        u16,
    pub status:         u16,
    pub revision_id:    u8,
    pub prog_if:        u8,
    pub subclass:       u8,
    pub class:          u8,
    pub cache_line_size:u8,
    pub latency_timer:  u8,
    pub header_type:    u8,
    pub bist:           u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct PCIDevice {
    pub header:         PCIHeader,
    pub bar0:           u32,
    pub bar1:           u32,
    pub bar2:           u32,
    pub bar3:           u32,
    pub bar4:           u32,
    pub bar5:           u32,
    pub cardbus_cus_ptr:u32,
    pub sub_sys_ven_id: u16,
    pub sub_sys_id:     u16,
    pub exp_rom_baddr:  u32,
    pub cap_ptr:        u8,
    pub res0:           [u8; 3],
    pub res1:           [u8; 4],
    pub ir_line:        u8,
    pub ir_pin:         u8,
    pub min_grant:      u8,
    pub max_latency:    u8
}

#[derive(Debug)]
pub struct PCIBus {
    pub configuration_space_address: *mut u8,
    pub device_list: [Option<*mut PCIDevice>; MAX_DEVICES],
}

impl PCIHeader {
    pub fn new(addr: *mut u8) -> Self {
        let pci_addr = addr as *mut PCIHeader;
        unsafe {
            PCIHeader {
                vendor_id:      addr_of_mut!((*pci_addr).vendor_id).read_volatile().to_le(),
                device_id:      addr_of_mut!((*pci_addr).device_id).read_volatile().to_le(),
                command:        addr_of_mut!((*pci_addr).command).read_volatile().to_le(),
                status:         addr_of_mut!((*pci_addr).status).read_volatile().to_le(),
                revision_id:    addr_of_mut!((*pci_addr).revision_id).read_volatile().to_le(),
                prog_if:        addr_of_mut!((*pci_addr).prog_if).read_volatile().to_le(),
                subclass:       addr_of_mut!((*pci_addr).subclass).read_volatile().to_le(),
                class:          addr_of_mut!((*pci_addr).class).read_volatile().to_le(),
                cache_line_size:addr_of_mut!((*pci_addr).cache_line_size).read_volatile().to_le(),
                latency_timer:  addr_of_mut!((*pci_addr).latency_timer).read_volatile().to_le(),
                header_type:    addr_of_mut!((*pci_addr).header_type).read_volatile().to_le(),
                bist:           addr_of_mut!((*pci_addr).bist).read_volatile().to_le(),
            }
        }
    }
}

impl PCIDevice {
    pub fn new(addr: *mut u8) -> Self {
        let pci_addr = addr as *mut PCIDevice;
        unsafe {
            PCIDevice {
                header:         PCIHeader::new(addr),
                bar0:           addr_of_mut!((*pci_addr).bar0).read_volatile().to_le(),
                bar1:           addr_of_mut!((*pci_addr).bar1).read_volatile().to_le(),
                bar2:           addr_of_mut!((*pci_addr).bar2).read_volatile().to_le(),
                bar3:           addr_of_mut!((*pci_addr).bar3).read_volatile().to_le(),
                bar4:           addr_of_mut!((*pci_addr).bar4).read_volatile().to_le(),
                bar5:           addr_of_mut!((*pci_addr).bar5).read_volatile().to_le(),
                cardbus_cus_ptr:addr_of_mut!((*pci_addr).cardbus_cus_ptr).read_volatile().to_le(),
                sub_sys_ven_id: addr_of_mut!((*pci_addr).sub_sys_ven_id).read_volatile().to_le(),
                sub_sys_id:     addr_of_mut!((*pci_addr).sub_sys_id).read_volatile().to_le(),
                exp_rom_baddr:  addr_of_mut!((*pci_addr).exp_rom_baddr).read_volatile().to_le(),
                cap_ptr:        addr_of_mut!((*pci_addr).cap_ptr).read_volatile().to_le(),
                res0:           addr_of_mut!((*pci_addr).res0).read_volatile(),
                res1:           addr_of_mut!((*pci_addr).res1).read_volatile(),
                ir_line:        addr_of_mut!((*pci_addr).ir_line).read_volatile().to_le(),
                ir_pin:         addr_of_mut!((*pci_addr).ir_pin).read_volatile().to_le(),
                min_grant:      addr_of_mut!((*pci_addr).min_grant).read_volatile().to_le(),
                max_latency:    addr_of_mut!((*pci_addr).max_latency).read_volatile().to_le()
            }
        }
    }
}

impl PCIBus {
    pub fn new(address: *mut u8) -> Self {
        PCIBus {
            configuration_space_address: address,
            device_list: [Option::None; MAX_DEVICES]
        }
    }

    pub fn enumerate(&mut self) {
        println!("Enumerating PCI...");
        unsafe {
            for bus in 0..256 {
                for device in 0..32 {
                    for function in 0..8 {
                        let device_addr = self.configuration_space_address.add(bus << 20 | device << 15 | function << 12) as *mut PCIHeader;
                        let vendor_id = addr_of_mut!((*device_addr).vendor_id).read_volatile().to_le();
                        if vendor_id != 0xFFFF {
                            let device_id = addr_of_mut!((*device_addr).device_id).read_volatile().to_le();
                            println!("Found device {vendor_id:0>4x?}:{device_id:0>4x?} at {bus:0>2x?}:{device:0>2x?}.{function:0>1x?}");
                            let header_type = addr_of_mut!((*device_addr).header_type).read_volatile().to_le();
                            if header_type == 0 {
                                for x in 0..self.device_list.len() {
                                    if self.device_list[x].is_none() {
                                        self.device_list[x] = Some(device_addr as *mut PCIDevice);
                                        break;
                                    }
                                }
                            } else {
                                panic!("Device not header type 0!");
                            }
                        }
                    }
                }
            }
        }
    }
}
