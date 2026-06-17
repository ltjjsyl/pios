# Raspberry Pi 5 Bring-up Notes

This project starts as a small learning kernel for Raspberry Pi 5.

## First boot target

The first milestone is intentionally narrow:

- boot a 64-bit AArch64 kernel image on Raspberry Pi 5
- initialize a stack and zero `.bss`
- jump into Rust `kernel_main`
- print early boot messages over UART

The expected serial settings are `115200 8N1`.

## Boot files

The boot partition should contain the Raspberry Pi firmware files, `boot/config.txt`, and the generated kernel image. Raspberry Pi 5 firmware defaults to `kernel_2712.img`, and this project also copies the same image to `kernel8.img` as a fallback.

## Current hardware assumptions

- Board: Raspberry Pi 5
- CPU mode: AArch64
- Primary console: UART
- UART base: `0x1f00030000`
- Kernel load address: `0x80000`

The UART address follows the Raspberry Pi 5 RP1 peripheral mapping and should be validated on real hardware during the first bring-up.
