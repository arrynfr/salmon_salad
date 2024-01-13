.option norvc
.section .text._start
.global _start
_start:
	wfi
	j		_start