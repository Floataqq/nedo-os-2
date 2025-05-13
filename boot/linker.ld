ENTRY (_start)

SECTIONS 
{
    . = 16K;

    .boot :
    {
      KEEP(*(.multiboot_header))
    }

    .text :
    {
      *(.text)
    }
}
