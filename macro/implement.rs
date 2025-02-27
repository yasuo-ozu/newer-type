use crate::ResultExt;
use derive_syn_parse::Parse;
use proc_macro2::TokenStream;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;
use template_quote::{quote, ToTokens};

fn check_ident(name: &'static str) -> impl Fn(parse::ParseStream) -> bool {
    move |input| match input.fork().parse::<Ident>() {
        Ok(ident) if ident == name => {
            input.parse::<Ident>().unwrap();
            true
        }
        _ => false,
    }
}

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
        let path = input.parse()?;
        if let Some(generics) = &mut generics {
            generics.1.where_clause = input.parse::<Option<WhereClause>>()?;
        }
        if input.is_empty() {
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
        path.segments
            .last_mut()
            .map(|seg| seg.arguments = PathArguments::None);
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
        input
            .parse()
            .map(Self::Enum)
            .or_else(|_| input.parse().map(Self::Struct))
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

pub trait Implement {
    fn implement(&self, arg: Argument) -> TokenStream;

    fn collect_implementors(&self) -> Vec<Implementor>;

    fn emit_impl(
        &self,
        self_val: &Expr,
        base_impls: impl IntoIterator<Item = (Implementor, Expr)>,
    ) -> TokenStream;

    fn ident(&self) -> &Ident;
    fn generics(&self) -> &Generics;
}

struct CollectImplementors(Vec<Implementor>);

const _: () = {
    use syn::visit::Visit;

    impl TargetDef {
        fn collect_implementors(&self) -> Vec<Implementor> {
            let mut this = CollectImplementors(Vec::new());
            match self {
                TargetDef::Enum(item_enum) => this.visit_item_enum(item_enum),
                TargetDef::Struct(item_struct) => this.visit_item_struct(item_struct),
            }
            this.0
        }
    }

    impl Visit<'_> for CollectImplementors {
        fn visit_attribute(&mut self, i: &Attribute) {
            if let Some(arg) = Argument::from_attr(i).unwrap_or_abort() {
                self.0.extend(arg.implementors)
            }
        }
    }
};

pub fn implement(arg: &Argument, target_def: &TargetDef) -> TokenStream {
    let imp: TokenStream = target_def
        .collect_implementors()
        .into_iter()
        .chain(arg.implementors.iter().cloned())
        .map(|implr| implr.emit_impl(target_def))
        .collect();
    quote! {
        #target_def
        #imp
    }
}
