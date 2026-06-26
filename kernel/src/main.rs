#![no_std]
#![no_main]

use core::arch::global_asm;
use core::fmt::Write;

use kernel::banner;
use kernel::console::ConsoleWriter;

mod arch;
mod board;
mod panic;

global_asm!(include_str!("arch/aarch64/boot.S"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut uart = board::raspi5::debug_uart::Uart::new();
    uart.init();

    let mut console = ConsoleWriter::new(&mut uart);
    let _ = writeln!(console, "{}", banner());
    let _ = writeln!(console, "Hello from kernel");
    let _ = writeln!(console, "board: Raspberry Pi 5 / BCM2712 / AArch64");
    let _ = writeln!(console, "stage: bring-up");
    let _ = writeln!(console, "console: Raspberry Pi 5 DEBUG UART");

    arch::aarch64::wait_forever()
}
