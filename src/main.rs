#![no_std]
#![no_main]

mod config;
#[macro_use] mod print;
use core::panic::PanicInfo;
use core::arch::asm;

mod efi;
use crate::efi::EfiSystemTable;

mod acpi;

fn serial_init() {
    let mut port: u16 = 0x3f8;
    let mut looptest: u8 = 0;
    unsafe {
        asm!("out dx, al", in("dx") port+1, in("al") 0x00 as u8);
        asm!("out dx, al", in("dx") port+3, in("al") 0x80 as u8);
	asm!("out dx, al", in("dx") port+0, in("al") 0x0C as u8);
        asm!("out dx, al", in("dx") port+1, in("al") 0x00 as u8);
        asm!("out dx, al", in("dx") port+3, in("al") 0x03 as u8);
        asm!("out dx, al", in("dx") port+2, in("al") 0xC7 as u8);
        asm!("out dx, al", in("dx") port+4, in("al") 0x0B as u8);
        asm!("out dx, al", in("dx") port+4, in("al") 0x1E as u8);
	asm!("out dx, al", in("dx") port+0, in("al") 0x41 as u8);
	asm!("in al, dx", out("al") looptest, in("dx") port+0);
	asm!("out dx, al", in("dx") port+4, in("al") 0x0F as u8);
    }
    if looptest == 0x41 {
	print!("It seems to work!\n\r");
    }
    else { panic!("No serial init!");}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);
    loop{}
}

#[no_mangle]
extern fn efi_main(_handle: u64, table: *mut EfiSystemTable) {
    efi::register_efi_system_table(table);
    efi::clear_screen();
    serial_init();
    if config::is_debug() {print!("You are running a debug build!\n\r");}
    print!("We're booting in UEFI mode ayyy!\n\r");
    println!();
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
    print!("{:x?}\n\r", guid);
    print!("{:?}\n\r", thingy);
    loop{};
}
