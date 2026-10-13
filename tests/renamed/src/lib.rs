//! Test-only crate: newer-type as a renamed dependency (`nt = { package =
//! "newer-type" }`). The default path to newer-type is resolved from this
//! crate's Cargo.toml.

#[nt::target]
pub trait Hello {
    fn hello(&self) -> u8;
}

impl Hello for u8 {
    fn hello(&self) -> u8 {
        *self + 1
    }
}

#[nt::implement(Hello)]
pub struct Wrap(pub u8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renamed_dependency() {
        assert_eq!(Wrap(2).hello(), 3);
    }
}
