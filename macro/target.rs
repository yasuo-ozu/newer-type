use proc_macro2::{Span, TokenStream};
use syn::*;
use template_quote::quote;

pub struct Argument {
    alternative: Option<Path>,
    newer_type: Path,
}

impl syn::parse::Parse for Argument {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let mut this = Argument {
            alternative: None,
            newer_type: parse_quote!(::newer_type),
        };
        while !input.is_empty() {
            let ident = input.parse::<Ident>()?;
            input.parse::<token::Eq>()?;
            match ident.to_string().as_str() {
                "alternative" => {
                    this.alternative = Some(input.parse()?);
                }
                "newer_type" => {
                    this.newer_type = input.parse()?;
                }
                _ => {
                    return Err(Error::new_spanned(&ident, "Unsupported argument"));
                }
            }
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(this)
    }
}

pub fn target(arg: Argument, input: ItemTrait) -> TokenStream {
    let temporal_mac_name = Ident::new(
        &format!("__newer_type_macro__{}", crate::random()),
        Span::call_site(),
    );
    quote! {
        #[macro_export]
        macro_rules! #temporal_mac_name {
            ($($t:tt)*) => {
                #{&arg.newer_type}::__implement_internal! {
                    /* Implementor */ ($($t)*)
                    /* trait_def */ #input,
                    /* alternative */ #{&arg.alternative},
                    /* newer_type */ #{&arg.newer_type}
                }
            }
        }
        #{&input.vis} use #temporal_mac_name as #{&input.ident};
        #input
    }
}
