.section .text._start
.global _start
.global _per_core_setup
_start:
	mrs x0, MPIDR_EL1
	and x0, x0, #0xFF
	cmp x0, xzr
	bne _per_core_setup

	mrs x19, CurrentEL
	lsr x19, x19, #2
	cmp x19, 1
	beq _per_core_setup

	adr x0, err_el_not_supported
	bl putstr
	add w0, w19, 0x30
	bl putchar
	adr x0, new_line
	bl putstr
	b .kend

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

putchar:
	ldr x9, =_serial_base
	strb w0, [x9]
	ret

putstr:
	mov x20, x30
	mov x10, x0
.loop:
	ldrb w0, [x10]
	bl putchar
	add x10, x10, #1
	cmp w0, 0
	bne .loop
	mov x30, x20
	ret

.data
err_el_not_supported:	.asciz "Can only boot in EL1. Current EL: "
new_line:				.asciz "\r\n"
