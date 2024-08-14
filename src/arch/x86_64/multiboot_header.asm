section .multiboot_header
header_start:
  dd 0xe85250d6
  dd 0
  dd header_end - header_start
  dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

  ; Framebuffer tag
  dw 5
  dw 20
  dd 1024
  dd 768
  dw 0
  dw 0

  ; End tag
  dw 0
  dw 8
header_end:
