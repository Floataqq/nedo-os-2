ENTRY(_start)

SECTIONS
{
    . = 4K;

    .boot :
    {
      KEEP(*(.multiboot_header))
    }

    . = 1M;

    .text : 
    { 
      *(.text .text.*) 
    }

    .bss :
    {
      *(.bss .bss.*)
    }
}

