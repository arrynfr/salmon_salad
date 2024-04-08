use core::arch::asm;

const PSCI_0_2_FN64_CPU_ON: isize = 0xc4000003;

extern "C" {
    static _num_cores: u8;
    static _per_core_setup: u8;
}

pub unsafe fn init_smp() {
    let current_core: u8;
    asm!("mrs x2, MPIDR_EL1", out("x2") current_core);
    
    if current_core == 0 {
        for init_core in 1..&_num_cores as *const u8 as u16 {
            // x0 is PSCI command as input and return code as output
            let mut return_code: isize = PSCI_0_2_FN64_CPU_ON; 
            asm!("hvc 0",
                inout("x0") return_code,
                in("x1") init_core,
                in("x2") &_per_core_setup as *const u8 as usize,
                in("x3") 0
            );
            if return_code != 0 {println!("Failed to initialize core {init_core} -> {return_code}");}
        }
    }
}
