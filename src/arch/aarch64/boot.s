.section .text._start
.global _start
.global _per_core_setup
.type _start, @function
.equ R_AARCH64_RELATIVE, 1027
_start:
    mov x14, x0
    mov x15, x1
    adrp x0, _base
    adrp x1, _rela_start
    add x1, x1, :lo12:_rela_start
    adrp x2, _rela_end
    add x2, x2, :lo12:_rela_end
    bl _relocate_binary

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

_el2_entry:
    # Enable access to timer registers in EL1
    mrs     x0, cnthctl_el2
    orr     x0, x0, #3
    msr     cnthctl_el2, x0     // allow EL1 system counter access
    msr     cntvoff_el2, xzr	// no virtual offset
    mov     x0, #(1 << 31)	// AArch64
    orr     x0, x0, #(1 << 1)	// SWIO
    msr     hcr_el2, x0
    mrs     x0, hcr_el2

    mrs x1, cptr_el2
    mov x0, #(3 << 20)
    orr x0, x1, x0
    msr cpacr_el1, x0

    b _el1_entry
    # If we get here something went wrong
    wfi
    b _el2_entry

# We don't run in EL3
_el3_entry:
    b _el1_entry
    wfi
    b _el3_entry

_el1_entry:
    mrs x0, MPIDR_EL1
    and x0, x0, #0xFF
    cmp x0, xzr
    bne _per_core_setup

_per_core_setup:
    #Enable floating point bits FPEN
    mrs x1, cpacr_el1
    mov x0, #(3 << 20)
    orr x0, x1, x0
    msr cpacr_el1, x0

    #Set up EL1 exception vector table (exception.s)
    adr x0, vector_table_el1
    msr vbar_el1, x0

    #Set up per core EL1 stack
    adrp x1, _stack_end
    add x1, x1, #4
    mrs x0, MPIDR_EL1
    and x0, x0, #0xFF
    ldr x3, =_stack_size
    mul x3, x0, x3
    sub x1, x1, x3
    mov sp, x1

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

.data
err_el_not_supported:	.asciz "Can only boot in EL1. Current EL: "
err_unknown_el:			.asciz "Cannot boot in unknown EL! Halting..."
new_line:				.asciz "\r\n"
