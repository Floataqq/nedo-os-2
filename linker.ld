SECTIONS
{
    . = 1M;

    .multiboot : {
        KEEP(*(.multiboot_header))
    }

    .text : { *(.text*) }
}

