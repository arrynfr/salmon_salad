use core::arch::asm;

use crate::arch::aarch64::driver::mmu::va_to_pa;

const PSCI_0_2_FN64_CPU_ON: isize = 0xc4000003;

extern "C" {
    static _num_cores: u8;
    static _per_core_setup: u8;
    static _el1_entry: u8;
}

pub unsafe fn init_smp() {
    let current_core: u8;
    asm!("mrs x2, MPIDR_EL1", out("x2") current_core);
    let start_addr = va_to_pa(&_el1_entry as *const u8 as usize).unwrap();
    if current_core == 0 {
        println!("Starting cores at: {:x}", start_addr);
        for init_core in 1..&_num_cores as *const u8 as u16 {
            // x0 is PSCI command as input and return code as output
            let return_code: isize;
            asm!("hvc 0",
                inout("x0") PSCI_0_2_FN64_CPU_ON => return_code,
                in("x1") init_core,
                in("x2") start_addr as usize,
                in("x3") 0,
                options(nostack, nomem)
            );
            if return_code != 0 { println!("Failed to initialize core {init_core}: {return_code}"); }
        }
    }
}

pub fn psci_shutdown() {
    const SYSTEM_OFF: u32 = 0x8400_0008;
    unsafe {
        asm!("hvc 0",
        in("x0") SYSTEM_OFF,
        options(nostack, nomem, noreturn));
    }
}
