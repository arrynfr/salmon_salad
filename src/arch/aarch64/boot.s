.section .text._start
.global _start
_start:
	#Enable floating point bits FPEN
	mrs    x1, cpacr_el1
	mov    x0, #(3 << 20)
	orr    x0, x1, x0
	msr    cpacr_el1, x0
	
	adr x1, hello_world
	bl putstr
	adrp x0, _stack_end
	mov sp, x0
	b _start_rust
	adr x1, goodbye_world
	bl putstr
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

.data
hello_world:    .asciz "Hello, aarch64 world!\r\n"
goodbye_world:  .asciz "Goodbye, aarch64 world!\r\n"
