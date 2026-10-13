use crate::ResultExt;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use syn::punctuated::Punctuated;
use syn::*;
use template_quote::{quote, ToTokens};

pub struct Output {
    pub implementor: Implementor,
    pub target_def: Adt,
}

impl syn::parse::Parse for Output {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let inner_stream;
        parenthesized!(inner_stream in input);
        let implementor = inner_stream.parse()?;
        let target_def = input.parse()?;
        let _ = input.parse::<Token![,]>();
        if input.is_empty() {
            Ok(Self {
                implementor,
                target_def,
            })
        } else {
            Err(input.error("Bad trailing tokens"))
        }
    }
}

impl template_quote::ToTokens for Output {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {(#{&self.implementor}) #{&self.target_def}});
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Implementor {
    pub generics: Option<(Token![for], Generics)>,
    pub path: Path,
}

impl core::fmt::Display for Implementor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", quote! {#{&self.path}})
    }
}

impl ToTokens for Implementor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
                #(if let Some((for_token, generics)) = &self.generics) {
                    #for_token #generics
                }
                #{&self.path}
                #(if let Some((_, generics)) = &self.generics) {
                    #{&generics.where_clause}
                }
        });
    }
}

impl syn::parse::Parse for Implementor {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut generics = if let Ok(for_token) = input.parse::<Token![for]>() {
            let lt_token = Some(input.parse::<Token![<]>()?);
            let mut params = Punctuated::new();
            while input.fork().parse::<Token![>]>().is_err() {
                let param = input.parse::<GenericParam>()?;
                params.push(param);
                if input.parse::<Token![,]>().is_err() {
                    break;
                }
            }
            let gt_token = Some(input.parse::<Token![>]>()?);
            Some((
                for_token,
                Generics {
                    lt_token,
                    params,
                    gt_token,
                    where_clause: None,
                },
            ))
        } else {
            None
        };
        let path: Path = input.parse()?;
        if let Some(seg) = path.segments.last() {
            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
                &seg.arguments
            {
                args.iter().for_each(|arg| match arg {
                    GenericArgument::AssocType(_)
                    | GenericArgument::AssocConst(_)
                    | GenericArgument::Constraint(_) => abort!(arg, "Not supported"),
                    _ => (),
                });
            }
        }
        if let Some(generics) = &mut generics {
            generics.1.where_clause = input.parse::<Option<WhereClause>>()?;
        }
        if generics.is_none() || input.is_empty() {
            Ok(Implementor { generics, path })
        } else {
            Err(input.error("Bad trailing tokens"))
        }
    }
}

impl Implementor {
    fn emit_impl(&self, target_def: &Adt, newer_type: &Path) -> TokenStream {
        let input = Output {
            implementor: self.clone(),
            target_def: target_def.clone(),
        };
        let mut path = self.path.clone();
        if let Some(seg) = path.segments.last_mut() {
            seg.arguments = PathArguments::None
        }
        // `Trait! { [Trait!, <newer_type>::__implement_internal!] (implementor)
        // adt }`
        announce::helper::ContList(vec![
            announce::helper::Cont::bang(path),
            announce::helper::Cont::bang(parse_quote!(#newer_type::__implement_internal)),
        ])
        .invoke(quote! { #input })
    }
}

/// `#[implement(Trait1, Trait2, .., newer_type = PATH)]`; `newer_type` is
/// optional and may be given in any position.
#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Argument {
    /// Path to the `newer_type` crate, for callers that reach it through a
    /// facade (default: resolved from the caller's `Cargo.toml`, see
    /// `newer_type_crate`).
    pub newer_type: Option<Path>,
    pub implementors: Punctuated<Implementor, Token![,]>,
}

impl syn::parse::Parse for Argument {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let mut newer_type = None;
        let mut implementors = Punctuated::new();
        while !input.is_empty() {
            if input.peek(Ident) && input.peek2(Token![=]) {
                let key: Ident = input.parse()?;
                if key != "newer_type" {
                    return Err(Error::new_spanned(key, "Unsupported argument"));
                }
                if newer_type.is_some() {
                    return Err(Error::new_spanned(key, "duplicate `newer_type` argument"));
                }
                input.parse::<Token![=]>()?;
                newer_type = Some(input.parse()?);
            } else {
                implementors.push(input.parse()?);
            }
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(Argument {
            newer_type,
            implementors,
        })
    }
}

impl ToTokens for Argument {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(p) = &self.newer_type {
            tokens.extend(quote!(newer_type = #p,));
        }
        self.implementors.to_tokens(tokens);
    }
}

impl Argument {
    pub fn from_attr(attr: &Attribute) -> Result<Option<Self>> {
        match &attr.meta {
            Meta::List(MetaList { path, tokens, .. }) if path.is_ident("implement") => {
                Ok(Some(parse2(tokens.clone())?))
            }
            _ => Ok(None),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Adt {
    Enum(ItemEnum),
    Struct(ItemStruct),
}

impl syn::parse::Parse for Adt {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        if input.fork().parse::<ItemEnum>().is_ok() {
            Ok(Self::Enum(input.parse()?))
        } else {
            Ok(Self::Struct(input.parse()?))
        }
    }
}

impl template_quote::ToTokens for Adt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Adt::Enum(item_enum) => item_enum.to_tokens(tokens),
            Adt::Struct(item_struct) => item_struct.to_tokens(tokens),
        }
    }
}

impl Adt {
    pub fn generics_mut(&mut self) -> &mut Generics {
        match self {
            Adt::Enum(item_enum) => &mut item_enum.generics,
            Adt::Struct(item_struct) => &mut item_struct.generics,
        }
    }

    fn collect_implementors(&mut self) -> Vec<Implementor> {
        let mut ret = Vec::new();
        let mut proceed_field = |field: &mut Field| {
            field.attrs = field
                .attrs
                .clone()
                .into_iter()
                .filter(|attr| {
                    if let Some(arg) = Argument::from_attr(attr).unwrap_or_abort() {
                        for implem in arg.implementors {
                            if !ret.iter().any(|a| a == &implem) {
                                ret.push(implem);
                            }
                        }
                        false
                    } else {
                        true
                    }
                })
                .collect();
        };
        match self {
            Adt::Enum(item_enum) => {
                for variant in item_enum.variants.iter_mut() {
                    for field in variant.fields.iter_mut() {
                        proceed_field(field);
                    }
                }
            }
            Adt::Struct(item_struct) => {
                for field in item_struct.fields.iter_mut() {
                    proceed_field(field);
                }
            }
        }
        ret
    }
}

pub fn implement(arg: &Argument, target_def: &Adt) -> TokenStream {
    let mut copied_target_def = target_def.clone();
    let newer_type = arg
        .newer_type
        .clone()
        .unwrap_or_else(crate::newer_type_crate);
    let imp: TokenStream = copied_target_def
        .collect_implementors()
        .into_iter()
        .chain(arg.implementors.iter().cloned())
        .map(|implr| implr.emit_impl(target_def, &newer_type))
        .collect();
    quote! {
        #copied_target_def
        #imp
    }
}
