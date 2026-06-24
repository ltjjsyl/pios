#[cfg(not(test))]
use core::fmt::Write;
#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
use kernel::console::ConsoleWriter;

#[cfg(not(test))]
use crate::arch;
#[cfg(not(test))]
use crate::board;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut uart = board::raspi5::uart::Uart::new();
    let mut console = ConsoleWriter::new(&mut uart);

    let _ = writeln!(console, "kernel panic: {}", info);

    arch::aarch64::wait_forever()
}
