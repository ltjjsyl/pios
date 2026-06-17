#![cfg_attr(not(test), no_std)]

pub mod console;

pub fn banner() -> &'static str {
    "raspi5-os: Rust learning kernel"
}

#[cfg(test)]
mod tests {
    use super::banner;

    #[test]
    fn banner_names_kernel() {
        assert!(banner().contains("raspi5-os"));
    }
}
