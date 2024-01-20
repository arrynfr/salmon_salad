_kernel_base = 0x40080000;
ENTRY(_kernel_base)

PHDRS
{
    segment_boot_core_stack PT_LOAD FLAGS(6);
    segment_code            PT_LOAD FLAGS(5);
    segment_data            PT_LOAD FLAGS(6);
}

SECTIONS
{
    . =  _kernel_base;
    .text :
    {
        KEEP(*(.text._start))
        *(.text*)
    } :segment_code
    .rodata : ALIGN(8) { *(.rodata*) } :segment_code
    .data : { *(.data*) } :segment_data
    .bss (NOLOAD) : ALIGN(16) {
        PROVIDE(_bss_start = .);
        KEEP(*(.sbss .sbss.*))
        KEEP(*(.bss .bss.*))
        . = ALIGN(16);
        PROVIDE(_bss_end = .);
    } :segment_data
    .stack (NOLOAD) :
    {
        . += 0x2800;
        _stack_end = .;
    } :segment_boot_core_stack
}