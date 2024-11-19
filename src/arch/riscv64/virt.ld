OUTPUT_ARCH( "riscv64" )
_kernel_base =  0x80000000;
_stack_size = 0x200000;
_alignment = 0x10000;
_heap_size = 128*1024*1024;

ENTRY(_kernel_base)

PHDRS
{
  segment_code PT_LOAD;
  segment_data PT_LOAD;
  bss PT_LOAD;
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
    
    .data : ALIGN(8) { 
        *(.data .data.*)
        PROVIDE(__global_pointer$ = .);
    } :segment_data
    
    .bss : ALIGN(8) {
        PROVIDE(_bss_start = .);
        KEEP(*(.sbss .sbss.*))
        KEEP(*(.bss .bss.*))
        . = ALIGN(16);
        PROVIDE(_bss_end = .);
    } :bss
    
    .stack (NOLOAD) : ALIGN(16)
    {
        . += _stack_size;
        _stack_end = .;
    }
    .heap (NOLOAD) : ALIGN(_alignment) {
        _heap_start = .;
        . += (_heap_size);
        _heap_end = .;
    }
}
