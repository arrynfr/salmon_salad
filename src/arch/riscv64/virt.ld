_kernel_base = 0x80000;
ENTRY(_kernel_base)
SECTIONS
{
    . =  _kernel_base;
    .text :
    {
        KEEP(*(.text._start))
    } :segment_code
    .eh_frame : {}
}
