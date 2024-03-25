_kernel_base = 0x40080000;
_stack_size = 0xC00000;
_num_cores = 2;
ENTRY(_kernel_base)

PHDRS
{
    segment_stack           PT_LOAD FLAGS(6);
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
        . += _stack_size*_num_cores;
        PROVIDE(_stack_end = .);
    } :segment_stack
}