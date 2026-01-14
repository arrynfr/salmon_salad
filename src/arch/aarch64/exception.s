.section .text
.macro save_ctx
    sub sp, sp, #288

    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    str x18, [sp, #144]
    stp x29, x30, [sp, #152]

    mrs x0, ELR_EL1
    mrs x1, ESR_EL1
    mrs x2, FAR_EL1
    stp x0, x1, [sp, #168]
    str x2, [sp, #184]

    mrs x0, SPSR_EL1
    str x0, [sp, #192]

    stp x19, x20, [sp, #200]
    stp x21, x22, [sp, #216]
    stp x23, x24, [sp, #232]
    stp x25, x26, [sp, #248]
    stp x27, x28, [sp, #264]
.endm

.macro restore_ctx
    ldr x0, [sp, #168]
    ldr x1, [sp, #192]
    msr ELR_EL1, x0
    msr SPSR_EL1, x1

    ldp x19, x20, [sp, #200]
    ldp x21, x22, [sp, #216]
    ldp x23, x24, [sp, #232]
    ldp x25, x26, [sp, #248]
    ldp x27, x28, [sp, #264]

    ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldr x18, [sp, #144]
    ldp x29, x30, [sp, #152]

    add sp, sp, #288
.endm

.macro call_sync handler
    save_ctx
    mov x0, sp
    bl \handler
    restore_ctx
    eret
.endm

.macro call_irq handler
    save_ctx
    mov x0, sp
    bl \handler
    restore_ctx
    eret
.endm

.balign 0x800
vector_table_el1:

curr_el_sp0_sync:
    b sync_entry

.balign 0x80
curr_el_sp0_irq:
    b irq_entry

.balign 0x80
curr_el_sp0_fiq:
    b irq_entry

.balign 0x80
curr_el_sp0_serror:
    b unhandled_entry

.balign 0x80
curr_el_spx_sync:
    b sync_entry

.balign 0x80
curr_el_spx_irq:
    b irq_entry

.balign 0x80
curr_el_spx_fiq:
    b irq_entry

.balign 0x80
curr_el_spx_serror:
    b unhandled_entry

.balign 0x80
lower_el_aarch64_sync:
    b sync_entry

.balign 0x80
lower_el_aarch64_irq:
    b irq_entry

.balign 0x80
lower_el_aarch64_fiq:
    b irq_entry

.balign 0x80
lower_el_aarch64_serror:
    b unhandled_entry

.balign 0x80
lower_el_aarch32_sync:
    b unhandled_entry

.balign 0x80
lower_el_aarch32_irq:
    b unhandled_entry

.balign 0x80
lower_el_aarch32_fiq:
    b irq_entry

.balign 0x80
lower_el_aarch32_serror:
    b unhandled_entry

sync_entry:
    call_sync exception_handler

irq_entry:
    call_irq irq_handler

unhandled_entry:
    call_sync unhandled_exception_vector
