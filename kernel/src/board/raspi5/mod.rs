pub mod uart {
    use core::ptr::{read_volatile, write_volatile};

    use kernel::console::Console;

    const UART0_BASE: usize = 0x1f00_030000;
    const UART_DR: usize = 0x00;
    const UART_FR: usize = 0x18;
    const UART_IBRD: usize = 0x24;
    const UART_FBRD: usize = 0x28;
    const UART_LCRH: usize = 0x2c;
    const UART_CR: usize = 0x30;
    const UART_IMSC: usize = 0x38;
    const UART_ICR: usize = 0x44;

    const FR_TXFF: u32 = 1 << 5;
    const CR_UARTEN: u32 = 1 << 0;
    const CR_TXE: u32 = 1 << 8;
    const CR_RXE: u32 = 1 << 9;
    const LCRH_WLEN_8: u32 = 0b11 << 5;
    const LCRH_FEN: u32 = 1 << 4;

    pub struct Uart;

    impl Uart {
        pub const fn new() -> Self {
            Self
        }

        pub fn init(&mut self) {
            unsafe {
                write_volatile(Self::reg(UART_CR), 0);
                write_volatile(Self::reg(UART_ICR), 0x7ff);
                write_volatile(Self::reg(UART_IBRD), 26);
                write_volatile(Self::reg(UART_FBRD), 3);
                write_volatile(Self::reg(UART_LCRH), LCRH_WLEN_8 | LCRH_FEN);
                write_volatile(Self::reg(UART_IMSC), 0);
                write_volatile(Self::reg(UART_CR), CR_UARTEN | CR_TXE | CR_RXE);
            }
        }

        fn reg(offset: usize) -> *mut u32 {
            (UART0_BASE + offset) as *mut u32
        }
    }

    impl Console for Uart {
        fn write_byte(&mut self, byte: u8) {
            unsafe {
                while read_volatile(Self::reg(UART_FR)) & FR_TXFF != 0 {}
                write_volatile(Self::reg(UART_DR), byte as u32);
            }
        }
    }
}
