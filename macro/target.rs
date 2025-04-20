use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use syn::*;
use template_quote::quote;
use type_leak::{Leaker, NotInternableError, Referrer};

pub struct Argument {
    alternative: Option<Path>,
    newer_type: Path,
}

impl syn::parse::Parse for Argument {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let mut alternative = None;
        let mut newer_type = parse_quote!(::newer_type);

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
                _ => {
                    return Err(Error::new_spanned(&ident, "Unsupported argument"));
                }
            }
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(Argument {
            alternative,
            newer_type,
        })
    }
}

fn emit_internal_trait(
    input: &ItemTrait,
    leak_trait_name: &Ident,
    referrer: &Referrer,
    nonce: u64,
) -> (TokenStream, TypeParamBound) {
    let self_type = Ident::new(&format!("__NewerTypeSelf{}", nonce), Span::call_site());
    let (_, ty_generics, where_clause) = input.generics.split_for_impl();
    let leak_trait_impl: Vec<_> = referrer
        .iter()
        .enumerate()
        .map(|(n, ty)| {
            (
                Ident::new(
                    &format!("__NewerTypeLeakedType_{}_{}", nonce, n),
                    Span::call_site(),
                ),
                ty.clone(),
            )
        })
        .collect();
    let mut impl_generics = input.generics.params.clone();
    for param in impl_generics.iter_mut() {
        match param {
            GenericParam::Type(type_param) => {
                type_param.eq_token = None;
                type_param.default = None;
            }
            GenericParam::Const(const_param) => {
                const_param.eq_token = None;
                const_param.default = None;
            }
            _ => (),
        }
    }
    impl_generics.push(GenericParam::Type(parse_quote!(#self_type)));
    (
        quote! {
            #[doc(hidden)]
            pub trait #leak_trait_name #{&input.generics} #where_clause {
                #(for (ident, _) in leak_trait_impl.iter()) {
                    type #ident: ?::core::marker::Sized;
                }
            }
            impl < #impl_generics > #leak_trait_name #ty_generics for #self_type
            where
                Self: #{&input.ident} #ty_generics,
                #{where_clause.map(|wc| &wc.predicates)}
            {
                #(for (ident, ty) in leak_trait_impl.iter()) {
                    type #ident = #ty;
                }
            }
        },
        parse_quote!(#leak_trait_name #ty_generics),
    )
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
    let mut leaker = Leaker::from_trait(
        &input_cloned,
    )
    .unwrap_or_else(|NotInternableError(span)| abort!(span, "cannot intern this element"; hint = "use absolute path instead"));
    leaker.reduce_roots();
    let referrer = leaker.finish();
    if let Some(unsafety) = &input.unsafety {
        abort!(&unsafety, "Cannot apply #[target] on unsafe traits");
    }
    let leak_trait_name = Ident::new(
        &format!("NewerTypeInternalTrait_{}", nonce),
        Span::call_site(),
    );
    let (internal_trait, supertrait) =
        emit_internal_trait(&input, &leak_trait_name, &referrer, nonce);
    let mut output = input.clone();
    output.supertraits.push(supertrait);
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
        output.unsafety = Some(Default::default());
        output.attrs.push(parse_quote!(#[doc = " # Safety"]));
        output.attrs.push(parse_quote!(#[doc = " "]));
        output
            .attrs
            .push(parse_quote!(#[doc = " should be implemented by [`newer_type::implement`]"]));
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
        #[allow(private_bounds)]
        #[allow(clippy::missing_safety_doc)]
        #output
        #internal_trait
    }
}
