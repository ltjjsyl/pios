.section .text.boot, "ax"
.global _start

_start:
    mrs x0, mpidr_el1
    and x0, x0, #0xff
    cbz x0, 2f

1:
    wfe
    b 1b

2:
    ldr x0, =__boot_stack_top
    mov sp, x0

    ldr x0, =__bss_start
    ldr x1, =__bss_end
3:
    cmp x0, x1
    b.hs 4f
    str xzr, [x0], #8
    b 3b

4:
    bl kernel_main

5:
    wfe
    b 5b
