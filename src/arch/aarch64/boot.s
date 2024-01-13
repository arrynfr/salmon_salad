.section .text._start
.global _start
_start:	
	adr x1, hello_world
	bl putstr
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
