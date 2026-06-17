use core::fmt;

pub trait Console {
    fn write_byte(&mut self, byte: u8);

    fn write_str_bytes(&mut self, text: &str) {
        for byte in text.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
    }
}

pub struct ConsoleWriter<'a, T: Console> {
    console: &'a mut T,
}

impl<'a, T: Console> ConsoleWriter<'a, T> {
    pub fn new(console: &'a mut T) -> Self {
        Self { console }
    }
}

impl<T: Console> fmt::Write for ConsoleWriter<'_, T> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.console.write_str_bytes(text);
        Ok(())
    }
}

pub struct LineBuffer<const N: usize> {
    bytes: [u8; N],
    len: usize,
}

impl<const N: usize> LineBuffer<N> {
    pub const fn new() -> Self {
        Self {
            bytes: [0; N],
            len: 0,
        }
    }

    pub fn push(&mut self, byte: u8) -> bool {
        if self.len == N {
            return false;
        }
        self.bytes[self.len] = byte;
        self.len += 1;
        true
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

#[cfg(test)]
mod tests {
    use super::{Console, LineBuffer};

    struct Capture {
        bytes: alloc::vec::Vec<u8>,
    }

    impl Console for Capture {
        fn write_byte(&mut self, byte: u8) {
            self.bytes.push(byte);
        }
    }

    extern crate alloc;

    #[test]
    fn console_expands_newline_to_crlf() {
        let mut capture = Capture {
            bytes: alloc::vec::Vec::new(),
        };

        capture.write_str_bytes("a\nb");

        assert_eq!(capture.bytes, b"a\r\nb");
    }

    #[test]
    fn line_buffer_rejects_bytes_when_full() {
        let mut buffer = LineBuffer::<2>::new();

        assert!(buffer.push(b'a'));
        assert!(buffer.push(b'b'));
        assert!(!buffer.push(b'c'));
        assert_eq!(buffer.as_bytes(), b"ab");
    }
}
