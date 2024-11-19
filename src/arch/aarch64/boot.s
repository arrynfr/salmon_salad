.section .text._start
.global _start
.global _per_core_setup
.global _el1_entry
.type _start, @function
.equ R_AARCH64_RELATIVE, 1027
_start:
#    ldr x11, =0xbe20e4000
#    add x12, x11, 4095
#    mov x13, 0xFFFFFFFFFFFFFFFF
#1:
#    str x13, [x11], #8
#    cmp x11, x12
#    bne 1b
#    ret

    #mrs x19, ID_AA64ISAR0_EL1
    #mrs x19, ID_AA64ISAR1_EL1
    #mrs x19, ID_AA64ISAR2_EL1
    #mrs x19, ID_AA64ISAR3_EL1
    #mrs x19, ID_ISAR5_EL1
    #
    #mrs x19, ID_AA64MMFR0_EL1
    #mrs x19, ID_AA64MMFR1_EL1
    #mrs x19, ID_AA64MMFR2_EL1
    #mrs x19, ID_AA64MMFR3_EL1
    #mrs x19, ID_AA64MMFR4_EL1
    #mrs x19, ID_MMFR4_EL1
    #mrs x19, ID_MMFR5_EL1
    #
    #mrs x19, ID_AA64PFR0_EL1
    #mrs x19, ID_AA64PFR1_EL1
    #mrs x19, ID_AA64PFR2_EL1
    #
    #mrs x19, ID_AA64DFR0_EL1
    #mrs x19, ID_DFR0_EL1

    mov x14, x0
    mov x15, x1

    mrs x19, CurrentEL
    lsr x19, x19, #2
    cmp x19, #1
    beq _el1_entry
    cmp x19, #2
    beq _el2_entry
    cmp x19, #3
    beq _el3_entry
    adr x0, err_unknown_el
    bl _putstr
    b .kend

# If we wanna use EL2 later on we need to
# set up the stack for EL2 and configure
# it appropriately. Currently it's only
# setting up EL1 access certain functions
# and then drops us down to EL1.
# Since we disable HVC instructions we can
# never return to EL2 for now.
_el2_entry:
    # Enable access to timer registers in EL1
    #mrs     x0, cnthctl_el2
    #orr     x0, x0, #3
    #msr     cnthctl_el2, x0     // allow EL1 system counter access
    #msr     cntvoff_el2, xzr	// no virtual offset

    #mrs     x0, hcr_el2
    #mov     x0, #(1 << 31)	    // AArch64
    #orr     x0, x0, #(1 << 1)	// SWIO
    #bic     x0, x0, #(1 << 29)  // HCD, disable HVC instructions
    #msr     hcr_el2, x0

    #mov x0, #(3 << 20) // Don't trap FP-Instructions
    #orr x0, x1, x0
    #msr cpacr_el1, x0

    #mov x0, 0b00101
    #orr x0, x0, (0b1111 << 6)
    #msr SPSR_EL2, x0

    #adr x0, vector_table_el1
    #msr vbar_el2, x0

    #adr x0, _el1_entry
    #msr ELR_EL2, x0

    #isb
    b _el1_entry
    #eret   //Todo eret, but this doesn't work atm on my mac...
            //Maybe get a usb-c cable that supports the serial output soon
    
    # If we get here something went wrong
_halt:
    wfi
    b _halt

# We don't run in EL3 so we do not
# set up anything up there.
# That means we do not do anything useful in EL3
# and just drop down to EL2
_el3_entry:
    msr SCTLR_EL2, xzr
    msr HCR_EL2, xzr

    mrs x0, SCR_EL3
    orr x0, x0, #(1 << 10) // AArch64 and 32 controlled by EL2
    orr x0, x0, #(1 << 0)  // EL1 and EL0 are non secure
    msr SCR_EL3, x0
    
    mov x0, xzr
    
    // Mask interrupts
    orr x0, x0, (0b1111 << 6)
    
    // Set the target EL2
    orr x0, x0, #0b0001
    orr x0, x0, #0b1000
    msr SPSR_EL3, x0

    adr x0, _el2_entry
    msr ELR_EL3, x0

    isb
    eret

    b _halt

_el1_entry:
    #Enable floating point bits FPEN
    mrs x1, cpacr_el1
    mov x0, #(3 << 20)
    orr x0, x1, x0
    msr cpacr_el1, x0

    mrs x0, MPIDR_EL1
    and x0, x0, #0xFF

    adrp x1, _stack_end //Temporary stack just to enable mmu
    add x1, x1, #16  //align stack at 16 byte boundary
    ldr x3, =_stack_size
    mul x3, x0, x3
    sub x1, x1, x3
    mov sp, x1
    
    mov sp, x1
    cmp x0, xzr
    bne 1f
    bl clear_bss
1:
    mrs x0, SCTLR_EL1
    and x0, x0, #1
    cmp x0, 1
    beq _after_mmu
    bl enable_mmu

    mrs x0, MPIDR_EL1
    and x0, x0, #0xFF
    adr x1, _per_core_setup
    bne 2f
    adr x1, _after_mmu
2:
    movk x1, 0xFFFF, lsl #48 // Identity mapping of kernel to 0xFFFF_*
    br x1 // Jump to paged kernel address

_after_mmu:
    adrp x0, _base
    adrp x1, _rela_start
    add x1, x1, :lo12:_rela_start
    adrp x2, _rela_end
    add x2, x2, :lo12:_rela_end
    bl _relocate_binary

_per_core_setup:
    #Set up per core EL1 stack
    adrp x1, _stack_end
    add x1, x1, #16  //align stack at 16 byte boundary
    mrs x0, MPIDR_EL1
    and x0, x0, #0xFF
    ldr x3, =_stack_size
    mul x3, x0, x3
    sub x1, x1, x3
    mov sp, x1

    #Set up EL1 exception vector table (exception.s)
    adr x0, vector_table_el1
    msr vbar_el1, x0

    mov x0, x14
    mov x1, x15
    isb
    dsb sy

    bl _start_rust // argc = x0; argv = x1
    ret

.kend:
    dsb sy
    wfi
    b		.kend

# in:	(x0 = base, x1 = rela_start, x2 = rela_end)
# mod:	(x12 = binary_address, x13 = addend_address,
# 		x9 = offset, x10 = type, x11 = addend)
.global _relocate_binary
.type _relocate_binary, @function
_relocate_binary:
    ldp x9, x10, [x1], 0x10
    ldr x11, [x1], 0x8

    cmp x10, R_AARCH64_RELATIVE
    bne 1f

    add x12, x0, x9
    add x13, x0, x11
    str x13, [x12]
    cmp x1, x2
    bne _relocate_binary
1:
    ret

# in:	(w0 = character)
# mod:	(x9 = UART_DATA_REGISTER)
.global _putchar
.type _putchar, @function
_putchar:
    ldr x9, =_serial_base
    cbz x9, 1f
    strb w0, [x9]
1:
    ret

# in:	(x0 = string_addr)
# mod:	(x20 = LR, x10 = string_addr, x0 = char)
.global _putstr
.type _putstr, @function
_putstr:
    mov x20, x30
    mov x10, x0
1:
    ldrb w0, [x10]
    bl _putchar

    add x10, x10, #1
    cmp w0, 0
    bne 1b

    mov x30, x20
    ret

.global _clear_bss
.type _clear_bss, @function
_clear_bss:
    ldr x7, =_bss_start
    ldr x8, =_bss_end
1:
    strb wzr, [x7], #1
    cmp x7, x8
    bls 1b
    ret

.data
err_unknown_el:			.asciz "Cannot boot in unknown EL! Halting..."
new_line:				.asciz "\r\n"

.section .user
.incbin "./test.bin"
