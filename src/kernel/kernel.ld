OUTPUT_ARCH( "riscv" )
ENTRY( _entry )

SECTIONS
{
    . = 0x0x80000000;

    .text : {
        *(.entry)
        *(.text .text.*)
        . = ALGIN(0x1000);
        _trampoline = .;
        (*trampsec)
        . = ALGIN(0x1000);
        PROVIDE(etext = .);
    }

    .rodata : {
        . = ALGIN(16);
        *(.srodata .srodata.*)
        . = ALGIN(16);
        *(.rodata . rodata.*)
    }

    .data : {
        . = ALGIN(16);
        *(.sbss .sbss.*)
        . = ALGIN(16);
        *(.bss .bss.*)
    }

    PROVIDE(end = .);
}