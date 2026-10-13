//! The inherited rename in a `[target.'cfg'.dependencies]` table.

#[nt_ws::announce::announce(crate = ::nt_ws::announce)]
pub trait Hello {
    fn hello(&self) -> u8;
}

impl Hello for u8 {
    fn hello(&self) -> u8 {
        *self + 1
    }
}

#[nt_ws::implement(Hello)]
pub struct Wrap(pub u8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_table() {
        assert_eq!(Wrap(6).hello(), 7);
    }
}
