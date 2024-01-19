.section .text._start
.global _start

.equ RTC_BASE,      0x40000000
.equ TIMER_BASE,    0x40004000

_start:
	csrr	t0, mhartid
	bnez	t0, .halt_cores
	csrw	satp, zero
    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop

	la sp, _stack_end
	call clear_bss

1:
	auipc   t0, %pcrel_hi(asm_trap_vector)
	addi    t0, t0, %pcrel_lo(1b) # it's important that the lowest bits are 0, otherwise we are in vectored mode
	csrrw   zero, mtvec, t0

	li		t0, (0b11 << 11) | (1 << 7) | (1 << 3) # MPP = 11; mpie = 1; mie = 1
	csrw	mstatus, t0

	# li		t3, (1 << 3) | (1 << 7) | (1 << 11)
	# csrw	mie, t3

	la		ra, .halt_cores
	la		t1, _start_rust
	csrw	mepc, t1
	
	mret
	
.halt_cores:
	wfi
	j .halt_cores

clear_bss:
	la a0, _bss_start
	la a1, _bss_end
	bgeu a0, a1, 2f
1:
	sd zero, (a0)
	addi a0, a0, 8
	bltu a0, a1, 1b
2:
	ret

putchar:
	li t1, 0x10000000
	sb a1, (t1)
	ret

puts:
	add t0, zero, ra
.loop:
	lb a1, (a0)
	addi a0, a0, 1
	jal putchar
	bne  a1, zero, .loop
	add ra, zero, t0
	ret
