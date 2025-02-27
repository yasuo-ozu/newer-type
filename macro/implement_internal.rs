use crate::implement::{
    Argument as ImplementArgument, Implementor, Output as ImplementOutput, TargetDef,
};
use crate::ResultExt;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use std::collections::HashMap;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::*;
use template_quote::quote;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Input {
    pub implementor: Implementor,
    pub target_def: TargetDef,
    pub trait_def: ItemTrait,
    pub alternative: Option<Path>,
    pub newer_type: Path,
}

impl syn::parse::Parse for Input {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let impl_output;
        parenthesized!(impl_output in input);
        let ImplementOutput {
            implementor,
            target_def,
        } = impl_output.parse()?;
        let trait_def = input.parse()?;
        input.parse::<Token![,]>()?;
        let alternative = if !input.peek(Token![,]) {
            Some(input.parse()?)
        } else {
            None
        };
        input.parse::<Token![,]>()?;
        let newer_type = input.parse()?;
        let _ = input.parse::<Token![,]>();
        if input.is_empty() {
            Ok(Self {
                implementor,
                target_def,
                trait_def,
                alternative,
                newer_type,
            })
        } else {
            Err(input.error("Bad trailing tokens"))
        }
    }
}

impl template_quote::ToTokens for Input {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_output = ImplementOutput {
            implementor: self.implementor.clone(),
            target_def: self.target_def.clone(),
        };
        tokens.extend(quote! {(#impl_output)});
        self.trait_def.to_tokens(tokens);
        <Token![,]>::default().to_tokens(tokens);
        self.alternative.to_tokens(tokens);
    }
}

fn merge_generic_params(
    args1: impl IntoIterator<Item = GenericParam, IntoIter: Clone>,
    args2: impl IntoIterator<Item = GenericParam, IntoIter: Clone>,
) -> impl Iterator<Item = GenericParam> {
    let it1 = args1.into_iter();
    let it2 = args2.into_iter();
    it1.clone()
        .filter(|arg| matches!(arg, GenericParam::Lifetime(_)))
        .chain(
            it2.clone()
                .filter(|arg| matches!(arg, GenericParam::Lifetime(_))),
        )
        .chain(
            it1.clone()
                .filter(|arg| matches!(arg, GenericParam::Const(_))),
        )
        .chain(
            it2.clone()
                .filter(|arg| matches!(arg, GenericParam::Const(_))),
        )
        .chain(
            it1.clone()
                .filter(|arg| matches!(arg, GenericParam::Type(_))),
        )
        .chain(
            it2.clone()
                .filter(|arg| matches!(arg, GenericParam::Type(_))),
        )
}

#[derive(Default)]
struct ModifyGenerics {
    lifetime_map: HashMap<Lifetime, Lifetime>,
    ident_map: HashMap<Ident, Ident>,
}

impl ModifyGenerics {
    fn append_lifetime(&mut self, lt: &mut Lifetime, nonce: u64) {
        let nlt = Lifetime::new(&format!("{}_newer_type_{}", &lt, nonce), lt.span());
        self.lifetime_map.insert(lt.clone(), nlt.clone());
        *lt = nlt;
    }

    fn append_ident(&mut self, ident: &mut Ident, nonce: u64) {
        let nident = Ident::new(&format!("{}_NEWER_TYPE_{}", &ident, nonce), ident.span());
        self.ident_map.insert(ident.clone(), nident.clone());
        *ident = nident;
    }
}

impl VisitMut for ModifyGenerics {
    fn visit_path_mut(&mut self, i: &mut Path) {
        match (i.segments.len(), i.segments.last_mut()) {
            (
                1,
                Some(PathSegment {
                    ident,
                    arguments: PathArguments::None,
                }),
            ) if i.leading_colon.is_none() => self.visit_ident_mut(ident),
            (_, Some(PathSegment { arguments, .. })) => {
                syn::visit_mut::visit_path_arguments_mut(self, arguments)
            }
            _ => (),
        }
    }

    fn visit_ident_mut(&mut self, i: &mut Ident) {
        if let Some(nident) = self.ident_map.get(i) {
            *i = nident.clone();
        }
    }

    fn visit_lifetime_mut(&mut self, i: &mut Lifetime) {
        if let Some(nlt) = self.lifetime_map.get(i) {
            *i = nlt.clone();
        }
    }
}

fn check_has_self_ty(ty: &Type) -> Option<Path> {
    struct CheckHasSelfTy(Option<Path>);
    impl Visit<'_> for CheckHasSelfTy {
        fn visit_type(&mut self, i: &Type) {
            match i {
                Type::Path(TypePath { qself: None, path }) if path.is_ident("Self") => {
                    self.0 = Some(path.clone());
                }
                _ => syn::visit::visit_type(self, i),
            }
        }
    }
    let mut checker = CheckHasSelfTy(None);
    checker.visit_type(ty);
    checker.0
}
fn check_is_self_ty(ty: &Type) -> Option<TokenStream> {
    match ty {
        Type::Reference(TypeReference {
            and_token,
            mutability,
            elem,
            ..
        }) => check_is_self_ty(elem.as_ref())
            .map(|ref_tokens| quote! (#and_token #mutability #ref_tokens)),
        Type::Path(TypePath {
            qself: None, path, ..
        }) if path.is_ident("Self") => Some(quote!()),
        _ => None,
    }
}

fn find_pred_param<'a>(
    args: impl IntoIterator<Item = &'a FnArg>,
) -> Option<(usize, Ident, TokenStream)> {
    let mut preds = args
        .into_iter()
        .enumerate()
        .filter_map(|(i, arg)| match arg.clone() {
            FnArg::Receiver(Receiver {
                reference,
                mutability,
                self_token,
                ..
            }) => Some((
                i,
                Ident::new("self", self_token.span()),
                quote!(#{reference.as_ref().map(|r| r.0)} #mutability),
            )),
            FnArg::Typed(PatType { pat, ty, .. }) => {
                if let (
                    Some(ref_tokens),
                    Pat::Ident(PatIdent {
                        ident,
                        subpat: None,
                        ..
                    }),
                ) = (check_is_self_ty(ty.as_ref()), pat.as_ref())
                {
                    Some((i, ident.clone(), ref_tokens))
                } else if let Some(reason) = check_has_self_ty(ty.as_ref()) {
                    abort!(
                        reason,
                        "Self type is not allowed here";
                        note = "acceptable types are `Self`, `&Self`, `&mut Self`, ..."
                    )
                } else {
                    None
                }
            }
        });
    if let Some((i, pred_ident, pred_ref_tokens)) = preds.next() {
        if let Some((_, reason, _)) = preds.next() {
            abort!(reason, "multiple `Self` type is not supported"; hint = pred_ident.span() => "first `Self` type is here");
        }
        Some((i, pred_ident, pred_ref_tokens))
    } else {
        None
    }
}

fn update_pat_names(pat: &mut Pat, f: &mut impl FnMut(Span) -> Ident) {
    match pat {
        Pat::Ident(PatIdent { subpat, .. }) => {
            if let Some((_, pat)) = subpat {
                update_pat_names(pat.as_mut(), f);
            }
        }
        Pat::Or(PatOr { cases, .. }) => {
            for case in cases {
                update_pat_names(case, f);
            }
        }
        Pat::Struct(PatStruct { fields, .. }) => {
            for FieldPat { pat, .. } in fields {
                update_pat_names(pat.as_mut(), f);
            }
        }
        Pat::Slice(PatSlice { elems, .. })
        | Pat::Tuple(PatTuple { elems, .. })
        | Pat::TupleStruct(PatTupleStruct { elems, .. }) => {
            for elem in elems {
                update_pat_names(elem, f);
            }
        }
        Pat::Paren(PatParen { pat, .. })
        | Pat::Reference(PatReference { pat, .. })
        | Pat::Type(PatType { pat, .. }) => {
            update_pat_names(pat.as_mut(), f);
        }
        pat @ Pat::Wild(_) => {
            let span = pat.span();
            *pat = Pat::Ident(PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: None,
                ident: f(span),
                subpat: None,
            });
        }
        Pat::Path(_) | Pat::Lit(_) | Pat::Rest(_) => (),
        _ => abort!(pat, "this pattern is not supported"),
    }
}

trait EmitImpl {
    fn emit_trait_fn(
        &self,
        trait_: &TokenStream,
        implementor: &Implementor,
        mut input: TraitItemFn,
        nonce: u64,
    ) -> TokenStream {
        for param in input.sig.inputs.iter_mut() {
            match param {
                FnArg::Typed(PatType { pat, .. }) => {
                    let mut cnt = 0usize;
                    update_pat_names(pat.as_mut(), &mut |span| {
                        cnt += 1;
                        Ident::new(&format!("__newer_type_arg_{}_{}", nonce, cnt), span)
                    })
                }
                _ => (),
            }
        }
        let (impl_generics, _, where_clause) = input.sig.generics.split_for_impl();
        let (pred_ix, pred_ident, pred_ref_tokens) = find_pred_param(&input.sig.inputs)
            .unwrap_or_else(|| abort!(&input.sig.inputs, "no `Self` type is found"; hint = "exact one `self` type is required in parameters"));
        if let ReturnType::Type(_, ty) = &input.sig.output {
            if let Some(reason) = check_has_self_ty(ty.as_ref()) {
                abort!(reason, "`Self` type is not allowed in return position");
            }
        }
        let body = self.emit_body(
            &parse_quote!(#pred_ident),
            &pred_ref_tokens,
            implementor,
            |pred_param| {
                quote! {
                    <_ as #trait_> :: #{&input.sig.ident} (
                        #(for (i, param) in input.sig.inputs.iter().enumerate()), {
                            #(if i == pred_ix) {
                                #pred_param
                            } #(else) {
                                #(if let FnArg::Receiver(Receiver{self_token, ..}) = param) {
                                    #self_token
                                }
                                #(if let FnArg::Typed(PatType {pat, ..}) = param) {
                                    #pat
                                }
                            }
                        }
                    )
                }
            },
        );
        quote! {
            #{&input.sig.constness}
            #{&input.sig.asyncness}
            #{&input.sig.unsafety}
            #{&input.sig.abi}
            fn #{&input.sig.ident} #impl_generics (
                #{&input.sig.inputs}
                #{&input.sig.variadic}
            ) #{&input.sig.output} {
                #body
            }
        }
    }

    fn emit_impl(&self, input: &Input) -> TokenStream {
        let self_val: Expr = parse_quote!(self);
        let mut path = input
            .alternative
            .clone()
            .unwrap_or(input.implementor.path.clone());
        path.segments
            .last_mut()
            .map(|last| last.arguments = PathArguments::None);
        let type_ty_generics = self.generics().split_for_impl().1;
        let mut trait_ty_generics = input
            .implementor
            .path
            .segments
            .last()
            .unwrap_or_else(|| abort!(&input.implementor, "Path without segments is not supported"))
            .arguments
            .clone();
        let mut impl_params = input
            .implementor
            .generics
            .as_ref()
            .map(|(_, g)| g.params.iter().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        let mut modifier: ModifyGenerics = Default::default();
        let nonce = crate::random();
        for p in impl_params.iter_mut() {
            match p {
                GenericParam::Lifetime(lt) => modifier.append_lifetime(&mut lt.lifetime, nonce),
                GenericParam::Type(TypeParam { ident, .. })
                | GenericParam::Const(ConstParam { ident, .. }) => {
                    modifier.append_ident(ident, nonce)
                }
            }
        }
        modifier.visit_path_arguments_mut(&mut trait_ty_generics);
        let where_clause = input
            .implementor
            .generics
            .as_ref()
            .and_then(|(_, g)| g.where_clause.clone())
            .map(|mut wc| {
                modifier.visit_where_clause_mut(&mut wc);
                wc
            });
        let impl_generics = merge_generic_params(self.generics().params.clone(), impl_params)
            .collect::<Punctuated<_, Token![,]>>();
        quote! {
            impl < #impl_generics > #path #trait_ty_generics for #{self.ident()} #type_ty_generics
            #where_clause
            {
                #(for item in &input.trait_def.items) {
                    #(if let TraitItem::Fn(tfn) = item) {
                        #{self.emit_trait_fn(&quote!(#path #trait_ty_generics), &input.implementor, tfn.clone(), nonce)}
                    }
                }
            }
        }
    }

    fn generics(&self) -> &Generics;

    fn ident(&self) -> &Ident;

    fn emit_body(
        &self,
        self_val: &Expr,
        ref_tokens: &TokenStream,
        implementor: &Implementor,
        f: impl FnMut(&Expr) -> TokenStream,
    ) -> TokenStream;
}

impl EmitImpl for ItemEnum {
    fn emit_body(
        &self,
        self_val: &Expr,
        ref_tokens: &TokenStream,
        implementor: &Implementor,
        mut f: impl FnMut(&Expr) -> TokenStream,
    ) -> TokenStream {
        let pred_param = Ident::new("__newer_type_pred_param", Span::call_site());
        quote! {
            match #self_val {
                #(for variant in &self.variants) {
                    #(let (n, _) = find_pred_field(implementor, &variant.fields)) {
                        Self::#{&variant.ident}
                        #(if let Fields::Named(_) = &variant.fields) {{
                            #(for (i, field) in variant.fields.iter().enumerate()) {
                                #(if i == n) { #{&field.ident}: #pred_param }
                                #(else) { #{&field.ident}: _ },
                            }
                        }}
                        #(if let Fields::Unnamed(_) = &variant.fields) {(
                            #(for (i, _) in variant.fields.iter().enumerate()), {
                                #(if i == n) { #pred_param }
                                #(else) {_}
                            }
                        )} => {#{ f(&parse_quote!(#pred_param)) }}
                    }
                }
            }
        }
    }

    fn generics(&self) -> &Generics {
        &self.generics
    }

    fn ident(&self) -> &Ident {
        &self.ident
    }
}

impl EmitImpl for ItemStruct {
    fn emit_body(
        &self,
        self_val: &Expr,
        ref_tokens: &TokenStream,
        implementor: &Implementor,
        mut f: impl FnMut(&Expr) -> TokenStream,
    ) -> TokenStream {
        let pred_param = Ident::new("__newer_type_pred_param", Span::call_site());
        let (n, pred_field) = find_pred_field(implementor, &self.fields);
        quote! {
            #(if let Fields::Named(_) = &self.fields) {
                let Self { #{&pred_field.ident}: #pred_param, ..} = #self_val;
            }
            #(if let Fields::Unnamed(_) = &self.fields) {
                let Self (
                    #(for (i, _) in self.fields.iter().enumerate()), {
                        #(if i == n) { #pred_param }
                        #(else) { _ }
                    }
                ) = #self_val;
            }
            #{ f(&parse_quote!(#pred_param)) }
        }
    }

    fn generics(&self) -> &Generics {
        &self.generics
    }

    fn ident(&self) -> &Ident {
        &self.ident
    }
}

impl Input {
    pub fn implement_internal(&self) -> TokenStream {
        match &self.target_def {
            TargetDef::Enum(item_enum) => item_enum.emit_impl(&self),
            TargetDef::Struct(item_struct) => item_struct.emit_impl(&self),
        }
    }
}

struct TraitMethod {
    slot: TraitItemFn,
    inputs: Vec<Expr>,
    pred_arg: (PredicatorQualifier, Expr),
}

enum PredicatorQualifier {
    None,
    And(Token![&]),
    AndMut(Token![&], Token![mut]),
}

fn find_pred_field(implementor: &Implementor, fields: &Fields) -> (usize, Field) {
    let pred_fields = fields
        .iter()
        .enumerate()
        .filter_map(|(i, field)| {
            field
                .attrs
                .iter()
                .any(|attr| match ImplementArgument::from_attr(attr) {
                    Ok(Some(arg)) => arg.implementors.iter().any(|im| im == implementor),
                    _ => false,
                })
                .then_some((i, field.clone()))
        })
        .collect::<Vec<_>>();
    let pred_fields = if pred_fields.is_empty() {
        if fields.len() == 1 {
            return (0, fields.iter().cloned().next().unwrap());
        } else {
            fields
                .iter()
                .cloned()
                .enumerate()
                .filter(|(_, field)| {
                    !field
                        .ident
                        .clone()
                        .map(|f| f.to_string().starts_with("_"))
                        .unwrap_or(true)
                })
                .collect::<Vec<_>>()
        }
    } else {
        pred_fields
    };
    match (pred_fields.len(), fields.len()) {
        (1, _) => pred_fields.into_iter().next().unwrap(),
        (0, 0) => abort!(
            fields,
            "No predicate found for implement {}",
            implementor;
            hint = "add any field here"
        ),
        (0, _) => abort!(fields, "No predicate found for implement {}", implementor),
        (n, _) => abort!(
            fields,
            "Cannot implement {} for {} predicates",
            implementor,
            n;
            note = pred_fields[0].1.span() => "first predicate is here";
            note = pred_fields[1].1.span() => "second predicate is here";
            note =? (n > 2).then_some("and one or more predicates");
            hint = "add #[implement({})] for any field", implementor;
        ),
    }
}

fn check_pred_type(ty: &Type) -> std::result::Result<PredicatorQualifier, ()> {
    match match ty {
        Type::Reference(TypeReference {
            and_token,
            mutability,
            elem,
            ..
        }) => match elem.as_ref() {
            Type::Path(TypePath { qself: None, path }) if path.is_ident("Self") => {
                Some((and_token.clone(), mutability.clone()))
            }
            _ => return Err(()),
        },
        Type::Path(TypePath { qself: None, path }) if path.is_ident("Self") => None,

        _ => return Err(()),
    } {
        Some((and_token, Some(mut_token))) => Ok(PredicatorQualifier::AndMut(and_token, mut_token)),
        Some((and_token, None)) => Ok(PredicatorQualifier::And(and_token)),
        _ => Ok(PredicatorQualifier::None),
    }
}

fn has_self(ty: &Type) -> Option<Type> {
    use syn::visit::Visit;
    struct Visitor(Option<Type>);
    impl Visit<'_> for Visitor {
        fn visit_type(&mut self, i: &Type) {
            match i {
                Type::Path(TypePath { qself: None, path }) if path.is_ident("Self") => {
                    self.0 = Some(i.clone())
                }
                _ => syn::visit::visit_type(self, i),
            }
        }
    }
    let mut visitor = Visitor(None);
    visitor.visit_type(ty);
    visitor.0
}

impl TraitMethod {
    fn new(slot: TraitItemFn) -> Self {
        let mut pred_args = Vec::new();
        let mut inputs = Vec::new();
        for (n, arg) in slot.sig.inputs.iter().enumerate() {
            match arg {
                FnArg::Receiver(Receiver {
                    reference,
                    mutability,
                    self_token,
                    ty,
                    ..
                }) => {
                    let expr: Expr = parse_quote!(#self_token);
                    let q = match (reference, mutability) {
                        (Some((and_token, _)), None) => PredicatorQualifier::And(and_token.clone()),
                        (Some((and_token, _)), Some(mut_token)) => {
                            PredicatorQualifier::AndMut(and_token.clone(), mut_token.clone())
                        }
                        _ => check_pred_type(ty.as_ref()).unwrap_or_else(|()|
                            abort!(ty, "This type is not supported"; note = "only `Self`, `&Self`, `&mut Self` are supported")
                        ),
                    };
                    inputs.push(expr.clone());
                    pred_args.push((q, expr));
                }
                FnArg::Typed(PatType { pat, ty, .. }) => {
                    let ident: Ident = match pat.as_ref() {
                        Pat::Ident(PatIdent { ident, .. }) => ident.clone(),
                        _ => Ident::new(&format!("__newer_type_arg{}", n), pat.span()),
                    };
                    let expr: Expr = parse_quote!(#ident);
                    if let Some(self_ty) = has_self(ty.as_ref()) {
                        pred_args.push((check_pred_type(ty.as_ref()).unwrap_or_else(|()|
                            abort!(ty, "This type is not supported"; note = "only `Self`, `&Self`, `&mut Self` are supported"; note = self_ty.span() => "since `Self` type is existing here")
                        ), expr.clone()));
                    }
                    inputs.push(expr)
                }
            }
        }
        match pred_args.len() {
            0 => abort!(&slot.sig.inputs, "No `Self` type or receiver found"),
            1 => (),
            n => {
                abort!(&slot.sig.inputs, "Multiple `Self` types or receivers found"; note = "required just one parameters, found {}", n;)
            }
        }
        Self {
            slot,
            inputs,
            pred_arg: pred_args.into_iter().next().unwrap(),
        }
    }

    fn emit_impl_item_fn(&self, f: impl FnMut((Expr, &[Expr])) -> TokenStream) -> TokenStream {
        todo!()
    }
}
