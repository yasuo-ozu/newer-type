use proc_macro::TokenStream as TokenStream1;
use proc_macro_error::{abort, proc_macro_error};
use syn::*;

mod cratepath;
mod implement;
mod implement_internal;

const PACKAGE: &str = "newer-type";

fn random() -> u64 {
    use std::hash::{BuildHasher, Hasher};
    std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish()
}

/// The path to the `newer-type` crate for generated code, when the caller gives
/// no `newer_type = ..`: the name under which the caller's `Cargo.toml` depends
/// on `newer-type` (renamed dependencies included, also those inherited from
/// `[workspace.dependencies]` with `workspace = true`). Within newer-type
/// itself, or when the manifest cannot be read (no Cargo), `::newer_type`. When
/// no dependency is found (the crate reaches newer-type through a facade),
/// `::newer_type` cannot resolve either, so this aborts with a hint.
fn newer_type_crate() -> Path {
    match proc_macro_crate::crate_name(PACKAGE) {
        Ok(proc_macro_crate::FoundCrate::Name(name)) => path_to(&name),
        Err(proc_macro_crate::Error::CrateNotFound { .. }) => {
            // proc-macro-crate does not follow `workspace = true`.
            let inherited = std::env::var_os("CARGO_MANIFEST_DIR").and_then(|dir| {
                cratepath::inherited_dependency_name(std::path::Path::new(&dir), PACKAGE)
            });
            match inherited {
                Some(name) => path_to(&name),
                None => abort!(
                    proc_macro2::Span::call_site(),
                    "newer_type: cannot tell the path to newer-type: no dependency on it was found in this crate's Cargo.toml (`workspace = true` entries included)";
                    hint = "if newer-type is reached through another crate (a facade), or the dependency is declared in a way that is not recognized, pass its path: `#[implement(newer_type = ::facade::newer_type, ..)]`"
                ),
            }
        }
        _ => parse_quote!(::newer_type),
    }
}

fn path_to(crate_name: &str) -> Path {
    let id = Ident::new(
        &crate_name.replace('-', "_"),
        proc_macro2::Span::call_site(),
    );
    parse_quote!(::#id)
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
