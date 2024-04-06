.section .text._start
.global _start
.global _per_core_setup
.type _start, @function
.equ R_AARCH64_RELATIVE, 1027
_start:
	mrs x0, MPIDR_EL1
	and x0, x0, #0xFF
	cmp x0, xzr
	bne _per_core_setup

	adrp x0, _base
    mov x20, x0
    adrp x1, _rela_start
    add x1, x1, :lo12:_rela_start
    adrp x2, _rela_end
    add x2, x2, :lo12:_rela_end

.relocate_binary:
	ldp x25, x26, [x1], 0x10
	ldr x27, [x1], 0x8
	cmp x26, R_AARCH64_RELATIVE
	bne .end_reloc
	add x22, x0, x25
	add x23, x0, x27
	str x23, [x22]
	cmp x1, x2
	bne .relocate_binary
	
.end_reloc:
	mrs x19, CurrentEL
	lsr x19, x19, #2
	cmp x19, 1
	beq _per_core_setup

	#adr x0, err_el_not_supported
	#bl _putstr
	#add w0, w19, 0x30
	#bl _putchar
	#adr x0, new_line
	#bl _putstr
	#b .kend

_per_core_setup:
	#Enable floating point bits FPEN
	mrs x1, cpacr_el1
	mov x0, #(3 << 20)
	orr x0, x1, x0
	msr cpacr_el1, x0

	#Set up EL1 exception vector table (exception.s)
	adr x0, vector_table_el1
	msr vbar_el1, x0

	adrp x1, _stack_end
	add x1, x1, 4
	mrs x0, MPIDR_EL1
	and x0, x0, #0xFF
	ldr x3, =_stack_size
	mul x3, x0, x3
	sub x1, x1, x3
	mov sp, x1

	isb
	dsb sy
	
	bl _start_rust // Core passed as func arg in x0

.kend:
	dsb sy
	wfi
	b		.kend

.global _putchar
.type _putchar, @function
_putchar:
	ldr x9, =_serial_base
	strb w0, [x9]
	ret

.global _putstr
.type _putstr, @function
_putstr:
	mov x20, x30
	mov x10, x0
.loop:
	ldrb w0, [x10]
	bl _putchar
	add x10, x10, #1
	cmp w0, 0
	bne .loop
	mov x30, x20
	ret

.data
err_el_not_supported:	.asciz "Can only boot in EL1. Current EL: "
new_line:				.asciz "\r\n"
