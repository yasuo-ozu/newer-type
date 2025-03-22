use crate::ResultExt;
use derive_syn_parse::Parse;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use syn::punctuated::Punctuated;
use syn::*;
use template_quote::{quote, ToTokens};

pub struct Output {
    pub implementor: Implementor,
    pub target_def: TargetDef,
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
        quote! {#{&self.path}}.fmt(f)
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
    fn emit_impl(&self, target_def: &TargetDef) -> TokenStream {
        let input = Output {
            implementor: self.clone(),
            target_def: target_def.clone(),
        };
        let mut path = self.path.clone();
        if let Some(seg) = path.segments.last_mut() {
            seg.arguments = PathArguments::None
        }
        quote! {
            #path! { #input }
        }
    }
}

#[derive(Parse, PartialEq, Eq, Debug, Hash, Clone)]
pub struct Argument {
    #[call(Punctuated::parse_terminated)]
    pub implementors: Punctuated<Implementor, Token![,]>,
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
pub enum TargetDef {
    Enum(ItemEnum),
    Struct(ItemStruct),
}

impl syn::parse::Parse for TargetDef {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        if input.fork().parse::<ItemEnum>().is_ok() {
            Ok(Self::Enum(input.parse()?))
        } else {
            Ok(Self::Struct(input.parse()?))
        }
    }
}

impl template_quote::ToTokens for TargetDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TargetDef::Enum(item_enum) => item_enum.to_tokens(tokens),
            TargetDef::Struct(item_struct) => item_struct.to_tokens(tokens),
        }
    }
}

impl TargetDef {
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
            TargetDef::Enum(item_enum) => {
                for variant in item_enum.variants.iter_mut() {
                    for field in variant.fields.iter_mut() {
                        proceed_field(field);
                    }
                }
            }
            TargetDef::Struct(item_struct) => {
                for field in item_struct.fields.iter_mut() {
                    proceed_field(field);
                }
            }
        }
        ret
    }
}

pub fn implement(arg: &Argument, target_def: &TargetDef) -> TokenStream {
    let mut copied_target_def = target_def.clone();
    let imp: TokenStream = copied_target_def
        .collect_implementors()
        .into_iter()
        .chain(arg.implementors.iter().cloned())
        .map(|implr| implr.emit_impl(target_def))
        .collect();
    quote! {
        #copied_target_def
        #imp
    }
}
