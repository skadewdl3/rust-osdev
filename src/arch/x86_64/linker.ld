ENTRY(start)

SECTIONS {
  . = 1M;

  .rodata : ALIGN(4K)
  {
    /* ensure that the multiboot header is at the beginning */
    KEEP(*(.multiboot_header))
    *(.rodata .rodata.*)
    . = ALIGN(4K);
  }

  .text : ALIGN(4K)
  {
    *(.text .text.*)
    . = ALIGN(4K);
  }

  .data :
  {
    *(.data .data.*)
    . = ALIGN(4K);
  }

  .bss :
  {
    *(.bss .bss.*)
    . = ALIGN(4K);
  }

  .got :
  {
    *(.got)
    . = ALIGN(4K);
  }

  .got.plt :
  {
    *(.got.plt)
    . = ALIGN(4K);
  }

  .data.rel.ro : ALIGN(4K) {
    *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
    . = ALIGN(4K);
  }

  .gcc_except_table : ALIGN(4K) {
    *(.gcc_except_table)
    . = ALIGN(4K);
  }

  .eh_frame : ALIGN(4K) {
    *(.eh_frame)
    . = ALIGN(4K);
  }

  linkme_TESTS : ALIGN(4K) {
    KEEP(*(linkme_TESTS))
  }

  linkm2_TESTS : ALIGN(4K) {
    KEEP(*(linkm2_TESTS))
  }

}
