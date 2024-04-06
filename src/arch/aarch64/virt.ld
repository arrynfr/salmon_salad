ENTRY(_start)
_stack_size = 0x200000;
_num_cores = 4;
_ram_size = 2048*1024*1024;
_serial_base = 0x9000000;

. = 0;

PHDRS
{
    segment_text            PT_LOAD FLAGS(5);
    segment_data            PT_LOAD FLAGS(6);
    segment_rodata          PT_LOAD FLAGS(4);
}

SECTIONS
{
    _base = .;
    .text : ALIGN(0x4000)
    {
        KEEP(*(.text._start))
        *(.text*)
        . = ALIGN(8);
        *(.got.plt)
        . = ALIGN(0x4000);
    } :segment_text
    .rodata : ALIGN(0x4000) { *(.rodata*) } :segment_rodata
    .rela.dyn : {
        _rela_start = .;
        *(.rela.*)
        _rela_end = .;
        . = ALIGN(0x4000);
    } :segment_rodata
    .data : ALIGN(0x4000) { 
        *(.data*)
        . = ALIGN(8);
        _got_start = .;
        *(.got)
        _got_end = .;
    } :segment_data
    .bss (NOLOAD) : ALIGN(0x4000) {
        _bss_start = .;
        *(.sbss .sbss.*)
        *(.bss .bss.*)
        *(.dynbss)
        *(COMMON)
        . = ALIGN(0x4000);
        _bss_end = .;
    } :segment_data
    .stack (NOLOAD) : ALIGN(0x4000) {
        _stack_start = .;
        . += (_stack_size*_num_cores);
        _stack_end = .;
    }
}
