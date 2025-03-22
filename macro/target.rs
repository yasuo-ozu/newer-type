use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use syn::*;
use template_quote::quote;
use type_leak::{Leaker, NotInternableError};

pub struct Argument {
    alternative: Option<Path>,
    newer_type: Path,
    implementor: Option<Path>,
}

impl syn::parse::Parse for Argument {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let mut alternative = None;
        let mut newer_type = parse_quote!(::newer_type);
        let mut implementor: Option<Path> = None;

        while !input.is_empty() {
            let ident = input.parse::<Ident>()?;
            input.parse::<token::Eq>()?;
            match ident.to_string().as_str() {
                "alternative" => {
                    alternative = Some(input.parse()?);
                }
                "newer_type" => {
                    newer_type = input.parse()?;
                }
                "implementor" => {
                    implementor = Some(input.parse()?);
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
        if let Some(implementor) = &implementor {
            if let Some(last_seg) = implementor.segments.iter().next_back() {
                if !last_seg.arguments.is_none() {
                    abort!(&last_seg.arguments, "Cannot specify arguments")
                }
            }
        }
        Ok(Argument {
            alternative,
            newer_type,
            implementor,
        })
    }
}

pub fn target(arg: Argument, input: ItemTrait) -> TokenStream {
    let mut input_cloned = input.clone();
    for item in input_cloned.items.iter_mut() {
        if let TraitItem::Fn(trait_item_fn) = item {
            trait_item_fn.default = None;
            trait_item_fn.semi_token = Some(Token![;](Span::call_site()));
        }
    }
    let nonce = crate::random();
    let crate_path = &arg.newer_type;
    let implementor = arg.implementor.clone().unwrap_or(parse_quote!(Dummy));
    let mut leaker = Leaker::from_trait(
        &input_cloned,
        Box::new(move |args| {
            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = args
            {
                let ty = type_leak::encode_generics_to_ty(&args);
                parse_quote!(#implementor <#ty>)
            } else {
                panic!()
            }
        }),
    )
    .unwrap_or_else(|NotInternableError(span)| abort!(span, "Not supported"));
    leaker.reduce_roots();
    let (item_impls, referrer) = leaker.finish(|n| parse_quote!(#crate_path::Repeater<#nonce, #n>));
    if !referrer.is_empty() && arg.implementor.is_none() {
        abort!(Span::call_site(), "Argument 'implementor' is required")
    }
    let mut output = input.clone();
    if let Some(mut alternative) = arg.alternative.clone() {
        let last_seg = alternative.segments.iter_mut().next_back().unwrap();
        let mut args = AngleBracketedGenericArguments {
            colon2_token: Default::default(),
            lt_token: Token![<](Span::call_site()),
            args: Default::default(),
            gt_token: Token![>](Span::call_site()),
        };
        for param in &input.generics.params {
            match param {
                GenericParam::Lifetime(LifetimeParam { lifetime, .. }) => {
                    args.args.push(GenericArgument::Lifetime(lifetime.clone()))
                }
                GenericParam::Type(TypeParam { ident, .. }) => {
                    args.args.push(GenericArgument::Type(parse_quote!(#ident)))
                }
                GenericParam::Const(ConstParam { ident, .. }) => {
                    args.args.push(GenericArgument::Const(parse_quote!(#ident)))
                }
            }
        }
        last_seg.arguments = PathArguments::AngleBracketed(args);
        output.colon_token = Some(Token![:](Span::call_site()));
        output.supertraits.push(TypeParamBound::Trait(TraitBound {
            paren_token: Default::default(),
            modifier: TraitBoundModifier::None,
            lifetimes: Default::default(),
            path: alternative,
        }));
        output.items = Vec::new();
    }

    let temporal_mac_name =
        Ident::new(&format!("__newer_type_macro__{}", nonce), Span::call_site());
    quote! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #temporal_mac_name {
            ($($t:tt)*) => {
                #{&arg.newer_type}::__implement_internal! {
                    /* Implementor */ ($($t)*)
                    /* trait_def */ #input,
                    /* alternative */ #{&arg.alternative},
                    /* newer_type */ #crate_path,
                    /* referrer */ #referrer,
                    /* nonce */ #nonce
                }
            }
        }
        #[doc(hidden)]
        #{&input.vis} use #temporal_mac_name as #{&input.ident};
        #(#item_impls)*
        #output
    }
}
