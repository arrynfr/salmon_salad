_kernel_base = 0x40080000;
_stack_size = 0x200000;
_num_cores = 4;
_ram_size = 2048*1024*1024;
_serial_base = 0x9000000;
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
    .rodata : ALIGN(16) { *(.rodata*) } :segment_code
    .data : ALIGN(16) { *(.data*) } :segment_data
    .bss (NOLOAD) : ALIGN(16) {
        PROVIDE(_bss_start = .);
        KEEP(*(.sbss .sbss.*))
        KEEP(*(.bss .bss.*))
        . = ALIGN(16);
        PROVIDE(_bss_end = .);
    } :segment_data
    .stack (NOLOAD) : ALIGN(16)
    {
        PROVIDE(_stack_start = .);
        . += _stack_size*_num_cores;
        PROVIDE(_stack_end = .);
    } :segment_stack
    .heap (NOLOAD) : ALIGN(16) {
        PROVIDE(_heap_start = .);
        . = _kernel_base+_ram_size;
        PROVIDE(_heap_end = .);
    }
}