.option norvc
.section .text._start
.global _start
_start:
	la x7, hello_world
	jal putstr
	la x7, goodbye_world
	jal putstr
	wfi
	j		_start

putchar:
	li x5, 0x10000000
	sb x6, (x5)
	ret

putstr:
	add x28, x0, x1
.loop:
	lb x6, (x7)
	addi x7, x7, 1
	jal putchar
	bne  x6, x0, .loop
	add x1, x0, x28
	ret

.data
hello_world:    .asciz "Hello, riscv64 world!\r\n"
goodbye_world:	.asciz "Goodbye, riscv64 world!\r\n"