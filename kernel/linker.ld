ENTRY(_start)

PHDRS
{
    limine_requests PT_LOAD;
    text            PT_LOAD;
    rodata          PT_LOAD;
    data            PT_LOAD;
}

SECTIONS
{
    . = 0xffffffff80000000;

    .limine_requests : ALIGN(16)
    {
        KEEP(*(.requests))
    } :limine_requests

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .text : ALIGN(16)
    {
        *(.text._start)
        *(.text*)
    } :text

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .rodata : ALIGN(16)
    {
        *(.rodata*)
    } :rodata

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .data : ALIGN(16)
    {
        *(.data*)
    } :data

    .bss : ALIGN(16)
    {
        *(COMMON)
        *(.bss*)
    } :data
}