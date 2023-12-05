#![no_std]
#![no_main]
#![feature(allow_internal_unstable)]

mod config;
#[macro_use] mod print;
use core::panic::PanicInfo;
mod efi;
mod acpi;
mod arch;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);
    loop{}
}

#[no_mangle]
extern fn efi_main(_handle: u64, table: *mut efi::EfiSystemTable) {
    efi::register_efi_system_table(table);
    efi::clear_screen();
    unsafe {
        arch::host::serial::serial_init();
    }
    //if config::IS_DEBUG {print!("You are running a debug build!\n\r");}
    //print!("We're booting in UEFI mode ayyy!\n\r");
    //println!();
    println!("Test");
    print!("AAAA");
    let thingy = acpi::Rsdp {
	signature: [0; 8],
	checksum: 0,
	oemid: [0; 6],
	revision: 0,
	rsdt_addr: 0,
	len: 0,
	xsdt_addr: 0 as *const usize,
	xchksum: 0,
	reserved: [0; 3]
    };

    let guid = efi::Guid(0x12345678, 0x1234, 0x1234, [0x1,0x2,0x3,0x4,0x5,0x6,0x7,0x8]);
    //let guid = efi::Guid(0x12345678_12345678_12345678_12345678);
    //print!("{:x?}\n\r", guid);
    //print!("{:?}\n\r", thingy);
    loop{};
}
