// Imports are only consumed by the gated `emit_traits!` block below, which is
// absent on toolchains before Rust 1.61 (where `Termination`/`ExitCode` are
// unstable).
#[allow(unused_imports)]
use crate::emit_traits;
#[allow(unused_imports)]
use newer_type::target;

// `std::process::Termination` and `std::process::ExitCode` were stabilized in
// Rust 1.61; omit the forwarding declaration on older toolchains.
#[rustversion::since(1.61)]
emit_traits! {
    #[implement_of(newer_type_std::process::Termination)]
    #[slot(std::process::ExitCode)]
    #[target(alternative = ::std::process::Termination)]
    pub trait Termination {
        fn report(self) -> ::std::process::ExitCode;
    }
}
