const MAX_DEVICES: u32 = 8192;

struct DeviceManager {
    devices: [Device; max_devices],
}

enum DeviceType {
    pcie,
    usb,
    other
}

struct Device {
    dev_type: DeviceType,
}

trait Interruptable {
    fn handle_interrupt(&self);
}

trait Device {

}