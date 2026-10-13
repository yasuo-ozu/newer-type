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

/// The path to the `newer-type` crate for generated code, when the caller gives
/// no `newer_type = ..`: the name under which the caller's `Cargo.toml` depends
/// on `newer-type` (renamed dependencies included). Within newer-type itself,
/// or when the manifest cannot be read (no Cargo), `::newer_type`. When the
/// manifest has no `newer-type` dependency (the crate reaches it through a
/// facade), `::newer_type` cannot resolve either, so this aborts with a hint.
fn newer_type_crate(attr: &str) -> Path {
    match proc_macro_crate::crate_name("newer-type") {
        Ok(proc_macro_crate::FoundCrate::Name(name)) => {
            let id = Ident::new(&name, proc_macro2::Span::call_site());
            parse_quote!(::#id)
        }
        Err(proc_macro_crate::Error::CrateNotFound { .. }) => abort!(
            proc_macro2::Span::call_site(),
            "newer_type: this crate's Cargo.toml has no `newer-type` dependency, so the path to newer-type is unknown";
            hint = "if newer-type is reached through another crate (a facade), pass its path: `#[{}(newer_type = ::facade::newer_type, ..)]`", attr
        ),
        _ => parse_quote!(::newer_type),
    }
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
pub fn target(arg: TokenStream1, input: TokenStream1) -> TokenStream1 {
    target::target(parse_macro_input!(arg), parse_macro_input!(input)).into()
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn implement(arg: TokenStream1, input: TokenStream1) -> TokenStream1 {
    implement::implement(&parse_macro_input!(arg), &parse_macro_input!(input)).into()
}

#[doc(hidden)]
#[proc_macro_error]
#[proc_macro]
pub fn __implement_internal(input: TokenStream1) -> TokenStream1 {
    parse_macro_input!(input as implement_internal::Input)
        .implement_internal()
        .into()
}
