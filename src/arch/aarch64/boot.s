.section .text._start
.global _start
.equ PSCI_0_2_FN64_CPU_ON, 0xc4000003
_start:
.stack_init:
	adrp x1, _stack_end
	add x1, x1, 4
	mrs x2, MPIDR_EL1
	and x2, x2, #0xFF
	ldr x3, =_stack_size
	mul x3, x2, x3
	sub x1, x1, x3
	mov sp, x1

	cmp x2, 0
	bne core_say_hello
	
	#Enable floating point bits FPEN
	mrs x1, cpacr_el1
	mov x0, #(3 << 20)
	orr x0, x1, x0
	msr cpacr_el1, x0
	isb
	
	b _start_rust

.core_fail:
	adr x1, bringup_failed
	bl putstr

.kend:
	wfi
	b		_start

putchar:
	mov x0, 0x09000000
	strb w2, [x0]
	ret

putstr:
	mov x8, 0
	add x8, x30, 0
.loop:
	ldrb w2, [x1]
	bl putchar
	add x1, x1, 1
	cmp w2, 0
	bne .loop
	mov x30, 0
	add x30, x8, 0
	ret

core_say_hello:
	adr x1, core_hello
	bl putstr
	mrs x2, MPIDR_EL1
	and x2, x2, #0xFF
	add x2, x2, 0x30
	bl putchar
	adr x1, new_line
	bl putstr
	b .kend

.data
hello_world:    .asciz "Hello, aarch64 world!\r\n"
goodbye_world:  .asciz "Goodbye, aarch64 world!\r\n"
bringup_failed: .asciz "Failed to bring up cores!\r\n"
core_hello: 	.asciz "Booted core "
new_line:		.asciz "\r\n"
