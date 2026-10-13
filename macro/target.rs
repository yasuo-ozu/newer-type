use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use syn::*;
use template_quote::quote;
use type_leak::{Leaker, NotInternableError, Referrer};

pub struct Argument {
    alternative: Option<Path>,
    newer_type: Path,
    repeater: Option<Path>,
}

impl syn::parse::Parse for Argument {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let mut alternative = None;
        let mut newer_type = None;
        let mut repeater = None;

        while !input.is_empty() {
            let ident = input.parse::<Ident>()?;
            input.parse::<token::Eq>()?;
            match ident.to_string().as_str() {
                "alternative" => {
                    alternative = Some(input.parse()?);
                }
                "newer_type" => {
                    newer_type = Some(input.parse()?);
                }
                "repeater" => {
                    repeater = Some(input.parse()?);
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
            newer_type: newer_type.unwrap_or_else(|| crate::newer_type_crate("target")),
            repeater,
        })
    }
}

/// Metadata attached to the announce entry of a `#[target]` trait
/// (`#[announce(meta = __target(..))]`), read back by `__implement_internal`.
pub struct TargetMeta {
    pub nonce: u64,
    pub repeater: Path,
    /// Leaked (context-dependent) types of the trait, in Repeater index
    /// order. It must be carried: `type_leak` orders them via a `HashSet`, so
    /// recomputing it in another macro invocation gives a different order.
    pub referrer: Referrer,
    pub alternative: Option<Path>,
}

impl TargetMeta {
    pub fn from_meta(meta: &announce::helper::Meta) -> Result<Self> {
        let mut kv = meta.kv()?;
        let m = TargetMeta {
            nonce: kv.require::<LitInt>("nonce")?.base10_parse()?,
            repeater: kv.require("repeater")?,
            referrer: kv.require("referrer")?,
            alternative: kv.get("alternative")?,
        };
        kv.finish().map(|_| m)
    }
}

/// The referrer (list of leaked, non-absolute types) of a trait.
pub fn trait_referrer(input: &ItemTrait) -> Referrer {
    let mut input_cloned = input.clone();
    for item in input_cloned.items.iter_mut() {
        if let TraitItem::Fn(trait_item_fn) = item {
            trait_item_fn.default = None;
            trait_item_fn.semi_token = Some(Token![;](Span::call_site()));
        }
    }
    let mut leaker = Leaker::from_trait(&input_cloned).unwrap_or_else(|NotInternableError(span)| abort!(span, "cannot intern this element"; hint = "use absolute path instead"));
    leaker.reduce_roots();
    leaker.finish()
}

fn emit_repeater_impl(
    input: &ItemTrait,
    trait_name: &Ident,
    referrer: &Referrer,
    repeater: &Path,
    define_repeater: bool,
    nonce: u64,
) -> TokenStream {
    let self_type = Ident::new(&format!("__NewerTypeSelf{}", nonce), Span::call_site());
    let (_, ty_generics, where_clause) = input.generics.split_for_impl();
    let generic_args = input
        .generics
        .params
        .iter()
        .map(|param| match param {
            GenericParam::Lifetime(LifetimeParam { lifetime, .. }) => {
                GenericArgument::Lifetime(lifetime.clone())
            }
            GenericParam::Type(TypeParam { ident, .. }) => {
                GenericArgument::Type(parse_quote!(#ident))
            }
            GenericParam::Const(ConstParam { ident, .. }) => {
                GenericArgument::Const(parse_quote!(#ident))
            }
        })
        .collect::<Vec<_>>();
    let mut impl_generics = input.generics.params.clone();
    impl_generics.push(GenericParam::Type(parse_quote!(#self_type)));
    let encoded_generics = type_leak::encode_generics_args_to_ty(&generic_args);
    let repeater_def = if define_repeater {
        quote! {
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            #{&input.vis} trait #repeater<
                T: ?::core::marker::Sized,
                const TRAIT_ID: ::core::primitive::u64,
                const NTH: ::core::primitive::usize
            > {
                type Type: ?::core::marker::Sized;
            }
        }
    } else {
        quote!()
    };
    let ret: TokenStream = referrer
        .iter()
        .enumerate()
        .map(|(n, ty)| {
            quote! {
                impl < #impl_generics > #repeater<#encoded_generics, #nonce, #n> for #self_type
                where
                    Self: #trait_name #ty_generics,
                    #{where_clause.map(|wc| &wc.predicates)}
                {
                    type Type = #ty;
                }
            }
        })
        .collect();
    quote!(#repeater_def #ret)
}

pub fn target(arg: Argument, input: ItemTrait) -> TokenStream {
    let nonce = crate::random();
    let crate_path = &arg.newer_type;
    let (repeater_path, define_repeater): (Path, bool) = if let Some(repeater) = &arg.repeater {
        (repeater.clone(), false)
    } else {
        let repeater_ident = Ident::new(
            &format!(
                "give_repeater_argument_to_this_target_attribute_macro_{}",
                nonce
            ),
            Span::call_site(),
        );
        (parse_quote!(#repeater_ident), true)
    };
    let ident = &input.ident;
    let referrer = trait_referrer(&input);
    let repeater_impl = emit_repeater_impl(
        &input,
        ident,
        &referrer,
        &repeater_path,
        define_repeater,
        nonce,
    );
    let meta = quote! {
        meta = __target(
            nonce = #nonce,
            repeater = #repeater_path,
            referrer = #referrer
            #(if let Some(alt) = &arg.alternative) { , alternative = #alt }
        )
    };
    // The full trait is announced without being emitted, wrapped in a module
    // of the same name: announce does not run its type-context check on
    // inline modules. That check would demand announce's own repeater (whose
    // shape differs from newer-type's user-visible `Repeater`) and it rejects
    // relative trait bounds outright. newer-type resolves leaked types with its
    // own Repeater (`referrer` in META), as before. The receiver unwraps the
    // module (`implement_internal::trait_of`).
    let announced = quote! {
        #[#crate_path::__private::announce::announce(
            crate = #crate_path::__private::announce,
            emit_item = false,
            #meta
        )]
        #{&input.vis} mod #ident { #input }
    };

    // With `alternative`, the item the user sees is an empty `unsafe trait
    // T: Alternative<..> {}`; the trampoline carries the full trait.
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
        output.unsafety = Some(Default::default());
        output.attrs.push(parse_quote!(#[doc = " # Safety"]));
        output.attrs.push(parse_quote!(#[doc = " "]));
        output
            .attrs
            .push(parse_quote!(#[doc = " should be implemented by [`newer_type::implement`]"]));
    }

    quote! {
        #announced
        #[allow(private_bounds)]
        #[allow(clippy::missing_safety_doc)]
        #output
        #repeater_impl
    }
}
