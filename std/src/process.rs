use crate::emit_traits;
use newer_type::target;

emit_traits! {
    #[implement_of(newer_type_std::process::Termination)]
    #[slot(std::process::ExitCode)]
    #[target(alternative = ::std::process::Termination)]
    pub trait Termination {
        fn report(self) -> ::std::process::ExitCode;
    }
}
