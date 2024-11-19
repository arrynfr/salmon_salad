use super::pci::PCIDevice;

pub struct RTL8139 {
    device: *mut PCIDevice,
    _io_space: *mut u32,
    memory_space: *mut u32
}

impl RTL8139 {
    pub fn new(dev: *mut PCIDevice, memory_space: usize, io_space: usize) -> Self {
        RTL8139 {
            device: dev,
            _io_space: io_space as *mut u32,
            memory_space: memory_space as *mut u32
        }
    }

    pub fn init() {
        unimplemented!("RTL driver is not implemented!");
    }
}