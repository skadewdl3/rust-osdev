section .multiboot_header
header_start:
  dd 0xe85250d6
  dd 0
  dd header_end - header_start
  dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

  ; Framebuffer tag
  align 8
framebuffer_tag_start:
  dw 5
  dw 1
  dw framebuffer_tag_end - framebuffer_tag_start
  dd 1024
  dd 768
  dw 0
framebuffer_tag_end:
header_end:
