//! Test-only crate: newer-type as a renamed dependency (`nt = { package =
//! "newer-type" }`). The generated code reaches newer-type through
//! `newer_type = ::nt`, as `::newer_type` does not resolve in this crate.

#[nt::announce::announce(crate = ::nt::announce)]
pub trait Hello {
    fn hello(&self) -> u8;
}

impl Hello for u8 {
    fn hello(&self) -> u8 {
        *self + 1
    }
}

#[nt::implement(Hello, newer_type = ::nt)]
pub struct Wrap(pub u8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renamed_dependency() {
        assert_eq!(Wrap(2).hello(), 3);
    }
}
