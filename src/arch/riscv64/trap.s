.section .text
.align 16 # This makes sure our trap vector is in direct mode which are the first 2 bits
.global asm_trap_vector
asm_trap_vector:
    csrrc  t0, mcause, zero
    bgez t0, print_err  # interrupt causes are less than zero
    slli t0, t0, 1      # shift off high bit
    srli t0, t0, 1
    j print_int

print_int:
        add t3, x0, 11
        bge t0, t3, 1f
        slli t0, t0, 3

        la a2, int_msg
        add a0, a2, t0
        call puts
1:
        mret


print_err:
        add t3, x0, 15
        bge t0, t3, 1f
        slli t0, t0, 3

        la a2, err_msg
        add a0, a2, t0
        call puts
1:
        mret

.section .rodata
err_msg:
    .string " IAML\r\n" # 0
    .string " IACF\r\n" # 1
    .string " IINS\r\n" # 2
    .string " BRKP\r\n" # 3
    .string " LAML\r\n" # 4
    .string " LAFT\r\n" # 5
    .string " STAM\r\n" # 6
    .string " STAF\r\n" # 7
    .string " ECUM\r\n" # 8
    .string " ECSM\r\n" # 9
    .string " RSV0\r\n" # 10
    .string " ECMM\r\n" # 11
    .string " INPF\r\n" # 12
    .string " LOPF\r\n" # 13
    .string " RSV1\r\n" # 14
    .string " STPF\r\n" # 15

int_msg:
    .string " IRV0\r\n" # 0
    .string " SSIN\r\n" # 1
    .string " IRV2\r\n" # 2
    .string " MSIN\r\n" # 3
    .string " IRV4\r\n" # 4
    .string " SVTI\r\n" # 5
    .string " IRV6\r\n" # 6
    .string " MMTI\r\n" # 7
    .string " IRV8\r\n" # 8
    .string " SVEI\r\n" # 9
    .string " IRV1\r\n" # 10
    .string " MMEI\r\n" # 11
