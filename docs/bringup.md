# Raspberry Pi 5 Bring-up Notes

This project starts as a small learning kernel for Raspberry Pi 5.

## First boot target

The first milestone is intentionally narrow:

- boot a 64-bit AArch64 kernel image on Raspberry Pi 5
- initialize a stack and zero `.bss`
- jump into Rust `kernel_main`
- print early boot messages over the Raspberry Pi 5 DEBUG UART

The expected serial settings are `115200 8N1`. For the first bring-up, use the Raspberry Pi 5 board DEBUG UART connector, not the 40-pin GPIO14/GPIO15 UART.

## Boot files

The boot partition should contain the Raspberry Pi firmware files, `boot/config.txt`, and the generated kernel image. Raspberry Pi 5 firmware defaults to `kernel_2712.img`, and this project also copies the same image to `kernel8.img` as a fallback.

## Build and IDE targets

The workspace does not set `aarch64-unknown-none` as the global default target. That keeps host tests and IDE analysis, including RustRover, in the normal Windows host context by default.

Use the build helper for Raspberry Pi 5 images:

```powershell
powershell -ExecutionPolicy Bypass -File .\tools\build.ps1
```

The helper passes `--target aarch64-unknown-none` explicitly, so the bare-metal kernel still uses the AArch64 linker script and panic handler. Host-only unit tests can run without a target override:

```powershell
cargo test -p kernel --lib
```

The kernel panic handler is compiled only outside Rust test builds. This avoids duplicate `panic_impl` diagnostics when host test tooling or IDE analysis uses `std`, while preserving the bare-metal panic path for Raspberry Pi images.

## Current hardware assumptions

- Board: Raspberry Pi 5
- CPU mode: AArch64
- Primary console: Raspberry Pi 5 DEBUG UART
- DEBUG UART base: `0x107d001000`
- 40-pin GPIO14/GPIO15 UART: not used by the first-stage console yet
- Kernel load address: `0x80000`

The first-stage console follows the same DEBUG UART path used by the Raspberry Pi bootloader logs. RP1 / 40-pin UART support is intentionally left for a later board-driver step.
