//! newer-type as a renamed dependency that is inherited from the workspace
//! (`nt-ws.workspace = true`): the default path to newer-type in the generated
//! code must follow it to `[workspace.dependencies]`.

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
    fn renamed_workspace_dependency() {
        assert_eq!(Wrap(2).hello(), 3);
    }
}
