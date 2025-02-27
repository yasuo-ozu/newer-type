use proc_macro::TokenStream as TokenStream1;
use proc_macro_error::{abort, proc_macro_error};
use syn::*;

mod implement;
mod implement_internal;
mod target;

fn random() -> u64 {
    use std::hash::{BuildHasher, Hasher};
    std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish()
}

trait ResultExt {
    type R;
    fn unwrap_or_abort(self) -> Self::R;
}

impl<T> ResultExt for syn::Result<T> {
    type R = T;

    fn unwrap_or_abort(self) -> Self::R {
        match self {
            Ok(ok) => ok,
            Err(e) => abort!(e.to_compile_error(), e),
        }
    }
}

#[proc_macro_error]
#[proc_macro_attribute]
#[proc_debug::proc_debug]
/// See `::newet_type::target`
pub fn target(arg: TokenStream1, input: TokenStream1) -> TokenStream1 {
    target::target(parse_macro_input!(arg), parse_macro_input!(input)).into()
}

#[proc_macro_error]
#[proc_macro_attribute]
#[proc_debug::proc_debug]
/// See `::newet_type::implement`
pub fn implement(arg: TokenStream1, input: TokenStream1) -> TokenStream1 {
    implement::implement(&parse_macro_input!(arg), &parse_macro_input!(input)).into()
}

#[doc(hidden)]
#[proc_macro_error]
#[proc_macro]
#[proc_debug::proc_debug]
pub fn __implement_internal(input: TokenStream1) -> TokenStream1 {
    parse_macro_input!(input as implement_internal::Input)
        .implement_internal()
        .into()
}
