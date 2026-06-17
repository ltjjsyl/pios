use core::fmt::Write;
use core::panic::PanicInfo;

use kernel::console::ConsoleWriter;

use crate::arch;
use crate::board;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut uart = board::raspi5::uart::Uart::new();
    let mut console = ConsoleWriter::new(&mut uart);

    let _ = writeln!(console, "kernel panic: {}", info);

    arch::aarch64::wait_forever()
}
