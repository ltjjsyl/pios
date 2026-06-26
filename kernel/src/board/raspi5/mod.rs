pub mod debug_uart {
    use core::ptr::{read_volatile, write_volatile};

    use kernel::console::Console;

    // Raspberry Pi 5 board DEBUG UART. The bootloader also writes its serial
    // diagnostics here, which makes it the best first bring-up console.
    const DEBUG_UART_BASE: usize = 0x107d_001000;
    const UART_DR: usize = 0x00;
    const UART_FR: usize = 0x18;
    const UART_CR: usize = 0x30;

    const FR_TXFF: u32 = 1 << 5;
    const CR_UARTEN: u32 = 1 << 0;
    const CR_TXE: u32 = 1 << 8;
    const CR_RXE: u32 = 1 << 9;
    const TX_READY_SPINS: usize = 10_000;

    pub struct Uart;

    impl Uart {
        pub const fn new() -> Self {
            Self
        }

        pub fn init(&mut self) {
            // Keep the bootloader's baud-rate and line-control setup. Early
            // bring-up only needs to make sure the UART and TX/RX paths are on.
            unsafe {
                write_volatile(Self::reg(UART_CR), CR_UARTEN | CR_TXE | CR_RXE);
            }
        }

        fn reg(offset: usize) -> *mut u32 {
            (DEBUG_UART_BASE + offset) as *mut u32
        }
    }

    impl Console for Uart {
        fn write_byte(&mut self, byte: u8) {
            unsafe {
                for _ in 0..TX_READY_SPINS {
                    if read_volatile(Self::reg(UART_FR)) & FR_TXFF == 0 {
                        break;
                    }
                }
                write_volatile(Self::reg(UART_DR), byte as u32);
            }
        }
    }
}
