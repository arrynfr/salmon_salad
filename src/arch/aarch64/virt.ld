ENTRY(_start)
_stack_size = 2*1024*1024;
_num_cores = 4;
_ram_size = 2048*1024*1024;
_serial_base = 0x9000000;
_alignment = 0x10000;

. = 0;

PHDRS
{
    segment_text            PT_LOAD FLAGS(5); /*0x1 PF_X; 0x2 PF_W; 0x3 PF_R*/
    segment_data            PT_LOAD FLAGS(6);
    segment_rodata          PT_LOAD FLAGS(4);
}

SECTIONS
{
    _base = .;
    .text : ALIGN(_alignment)
    {
        KEEP(*(.text._start))
        *(.text*)
        . = ALIGN(8);
        *(.got.plt)
        _user_start = .;
        *(.user*)
    } :segment_text
    .rodata : ALIGN(_alignment) { *(.rodata*) } :segment_rodata
    .rela.dyn : ALIGN(_alignment) {
        _rela_start = .;
        *(.rela.*)
        _rela_end = .;
    } :segment_rodata
    .data : ALIGN(_alignment) { 
        *(.data*)
        . = ALIGN(8);
        _got_start = .;
        *(.got)
        _got_end = .;
    } :segment_data
    .bss (NOLOAD) : ALIGN(_alignment) {
        _bss_start = .;
        *(.sbss .sbss.*)
        *(.bss .bss.*)
        *(.dynbss)
        *(COMMON)
        . = ALIGN(_alignment);
        _bss_end = .;
    } :segment_data
    .stack (NOLOAD) : ALIGN(_alignment) {
        _stack_start = .;
        . += (_stack_size*_num_cores);
        _stack_end = .;
        . += (_stack_size*_num_cores);
        _user_stack = .;
    }
}
