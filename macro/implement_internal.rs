use crate::implement::{
    Adt, Argument as ImplementArgument, Implementor, Output as ImplementOutput,
};
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use std::collections::{HashMap, HashSet};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::*;
use template_quote::quote;
use type_leak::Referrer;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Input {
    pub implementor: Implementor,
    pub adt: Adt,
    pub trait_def: ItemTrait,
    pub alternative: Option<Path>,
    pub newer_type: Path,
    pub referrer: Referrer,
    pub repeater: Path,
    pub nonce: u64,
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
        input.parse::<Token![,]>()?;
        let referrer = input.parse()?;
        input.parse::<Token![,]>()?;
        let repeater = input.parse()?;
        input.parse::<Token![,]>()?;
        let nonce: LitInt = input.parse()?;
        let _ = input.parse::<Token![,]>();
        if input.is_empty() {
            Ok(Self {
                implementor,
                adt: target_def,
                trait_def,
                alternative,
                newer_type,
                referrer,
                repeater,
                nonce: nonce.base10_parse()?,
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
            target_def: self.adt.clone(),
        };
        tokens.extend(quote! {(#impl_output)});
        self.trait_def.to_tokens(tokens);
        <Token![,]>::default().to_tokens(tokens);
        self.alternative.to_tokens(tokens);
        <Token![,]>::default().to_tokens(tokens);
        self.referrer.to_tokens(tokens);
        <Token![,]>::default().to_tokens(tokens);
        self.repeater.to_tokens(tokens);
        <Token![,]>::default().to_tokens(tokens);
        tokens.extend(quote! {#{self.nonce}})
    }
}

fn merge_generic_params<I1, I2>(args1: I1, args2: I2) -> impl Iterator<Item = GenericParam>
where
    I1: IntoIterator<Item = GenericParam>,
    I2: IntoIterator<Item = GenericParam>,
    I1::IntoIter: Clone,
    I2::IntoIter: Clone,
{
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

#[derive(Default, Clone, Debug)]
struct CorrectAssocTys(HashSet<Ident>);

impl visit::Visit<'_> for CorrectAssocTys {
    fn visit_type(&mut self, i: &Type) {
        if let Type::Path(TypePath {
            qself: None,
            path:
                Path {
                    leading_colon: None,
                    segments,
                },
        }) = i
        {
            if segments.len() == 2
                && &segments[0].ident == "Self"
                && segments[0].arguments == PathArguments::None
            {
                self.0.insert(segments[1].ident.clone());
            }
        } else {
            syn::visit::visit_type(self, i);
        }
    }
}

#[derive(Default, Clone, Debug)]
struct ModifyGenerics {
    lifetime_map: HashMap<Lifetime, Lifetime>,
    const_map: HashMap<Ident, Expr>,
    type_map: HashMap<Ident, Type>,
}

impl ModifyGenerics {
    fn append_lifetime(&mut self, lt: &mut Lifetime, nonce: u64) {
        let nlt = Lifetime::new(&format!("{}_newer_type_{}", &lt, nonce), lt.span());
        self.lifetime_map.insert(lt.clone(), nlt.clone());
        *lt = nlt;
    }

    fn append_type(&mut self, ident: &mut Ident, nonce: u64) {
        let nident = Ident::new(
            &format!("NewerTypeTypeParam{}Of{}", &ident, nonce),
            ident.span(),
        );
        self.type_map.insert(ident.clone(), parse_quote!(#nident));
        *ident = nident;
    }

    fn append_const(&mut self, ident: &mut Ident, nonce: u64) {
        let nident = Ident::new(
            &format!("NewerTypeTypeParam{}Of{}", &ident, nonce),
            ident.span(),
        );
        self.const_map.insert(ident.clone(), parse_quote!(nident));
        *ident = nident;
    }

    fn filter_generics(&self, generics: &Generics) -> Self {
        let mut m = self.clone();
        for param in &generics.params {
            match param {
                GenericParam::Lifetime(LifetimeParam { lifetime, .. }) => {
                    m.lifetime_map.remove(lifetime);
                }
                GenericParam::Type(TypeParam { ident, .. }) => {
                    m.type_map.remove(ident);
                }
                GenericParam::Const(ConstParam { ident, .. }) => {
                    m.const_map.remove(ident);
                }
            }
        }
        m
    }
}

impl VisitMut for ModifyGenerics {
    fn visit_attribute_mut(&mut self, i: &mut syn::Attribute) {
        if let Ok(Some(mut implr_arg)) = ImplementArgument::from_attr(i) {
            for implr in implr_arg.implementors.iter_mut() {
                if let Some((_, generics)) = &implr.generics {
                    let mut m = self.filter_generics(generics);
                    visit_mut::visit_path_mut(&mut m, &mut implr.path);
                } else {
                    visit_mut::visit_path_mut(self, &mut implr.path);
                }
            }
            *i = parse_quote!(#[implement(#implr_arg)]);
        }
    }

    fn visit_trait_item_fn_mut(&mut self, i: &mut syn::TraitItemFn) {
        for attr in &mut i.attrs {
            self.visit_attribute_mut(attr);
        }
        self.visit_signature_mut(&mut i.sig);
        // ignore default body
    }

    fn visit_type_mut(&mut self, i: &mut Type) {
        match i {
            Type::Path(TypePath { qself, path }) if qself.is_none() => {
                if let Some(ident) = path.get_ident() {
                    if let Some(ntyp) = self.type_map.get(ident) {
                        *i = ntyp.clone();
                        return;
                    }
                }
            }
            _ => (),
        }
        visit_mut::visit_type_mut(self, i);
    }

    fn visit_expr_mut(&mut self, i: &mut Expr) {
        match i {
            Expr::Path(ExprPath { qself, path, .. }) if qself.is_none() => {
                if let Some(ident) = path.get_ident() {
                    if let Some(nconst) = self.const_map.get(ident) {
                        *i = nconst.clone();
                        return;
                    }
                }
            }
            _ => (),
        }
        visit_mut::visit_expr_mut(self, i);
    }

    fn visit_lifetime_mut(&mut self, i: &mut Lifetime) {
        if let Some(nlt) = self.lifetime_map.get(i) {
            *i = nlt.clone();
        }
    }

    fn visit_item_impl_mut(&mut self, i: &mut ItemImpl) {
        let mut m = self.filter_generics(&i.generics);
        visit_mut::visit_item_impl_mut(&mut m, i);
    }

    fn visit_item_trait_mut(&mut self, i: &mut ItemTrait) {
        let mut m = self.filter_generics(&i.generics);
        visit_mut::visit_item_trait_mut(&mut m, i);
    }

    fn visit_item_struct_mut(&mut self, i: &mut ItemStruct) {
        let mut m = self.filter_generics(&i.generics);
        visit_mut::visit_item_struct_mut(&mut m, i);
    }

    fn visit_item_enum_mut(&mut self, i: &mut ItemEnum) {
        let mut m = self.filter_generics(&i.generics);
        visit_mut::visit_item_enum_mut(&mut m, i);
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
) -> Vec<(usize, Ident, TokenStream)> {
    args.into_iter()
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
        })
        .collect()
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

trait EmitImpl: Sized + Clone + template_quote::ToTokens {
    fn emit_trait_const(
        &self,
        trait_: &Path,
        implementor: &Implementor,
        input: &TraitItemConst,
    ) -> TokenStream {
        let ptyps = self.get_predicate_types(implementor);
        if ptyps.len() != 1 {
            abort!(&implementor.path, "cannot implement this trait to enum"; note = input.span() => "because the trait has associated const");
        }
        quote! {
            const #{&input.ident} : #{&input.ty} = <#(#ptyps)* as #trait_>::#{&input.ident};
        }
    }

    fn emit_trait_fn(
        &self,
        trait_: &Path,
        implementor: &Implementor,
        mut input: TraitItemFn,
        nonce: u64,
        leaked_ty_visitor: &mut impl VisitMut,
    ) -> TokenStream {
        for param in input.sig.inputs.iter_mut() {
            if let FnArg::Typed(PatType { pat, .. }) = param {
                let mut cnt = 0usize;
                update_pat_names(pat.as_mut(), &mut |span| {
                    cnt += 1;
                    Ident::new(&format!("__newer_type_arg_{nonce}_{cnt}"), span)
                })
            }
        }
        let (impl_generics, _, where_clause) = input.sig.generics.split_for_impl();
        let preds = find_pred_param(&input.sig.inputs);
        if preds.is_empty() {
            abort!(&input.sig.inputs, "no `Self` type is found"; hint = "exact one `self` type is required in parameters");
        } else if preds.len() > 1 && self.get_predicate_types(implementor).is_empty() {
            // self is enum
            abort!(&preds[1].1, "multiple `Self` type is not supported"; hint = preds[0].1.span() => "first `Self` type is here");
        }
        if let ReturnType::Type(_, ty) = &input.sig.output {
            if let Some(reason) = check_has_self_ty(ty.as_ref()) {
                abort!(reason, "`Self` type is not allowed in return position");
            }
        }
        let mut sig = input.sig.clone();
        leaked_ty_visitor.visit_signature_mut(&mut sig);
        let body = self.emit_body(&preds, implementor, |pred_params| {
            let process_pat = |mut pat: Pat| -> Pat{
                struct PatVisitor;
                impl syn::visit_mut::VisitMut for PatVisitor {
                    fn visit_pat_ident_mut(&mut self, i: &mut PatIdent) {
                        i.mutability = None;
                    }
                }
                PatVisitor.visit_pat_mut(&mut pat);
                pat
            };
            quote! {
                <_ as #trait_> :: #{&sig.ident} (
                    #(for (i, param) in sig.inputs.iter().enumerate()), {
                        #(if let Some((_, pred_param)) = preds.iter().zip(pred_params).find(|((n, _, _), _)| &i == n)) {
                            #pred_param
                        }
                        #(else) {
                            #(if let FnArg::Receiver(Receiver{self_token, ..}) = param) {
                                #self_token
                            }
                            #(if let FnArg::Typed(PatType {pat, ..}) = param) {
                                #{process_pat(*pat.clone())}
                            }
                        }
                    }
                )
            }
        });
        quote! {
            #{&sig.constness}
            #{&sig.asyncness}
            #{&sig.unsafety}
            #{&sig.abi}
            fn #{&sig.ident} #impl_generics (
                #{&sig.inputs}
                #{&sig.variadic}
            ) #{&sig.output} #{where_clause} {
                #body
            }
        }
    }

    fn get_predicate_types(&self, implementor: &Implementor) -> Vec<Type>;

    fn emit_impl(
        &self,
        input: &Input,
        nonce: u64,
        leaked_ty_visitor: &mut impl VisitMut,
    ) -> TokenStream {
        let mut path = input
            .alternative
            .clone()
            .unwrap_or(input.implementor.path.clone());
        if let Some(last) = path.segments.last_mut() {
            last.arguments = PathArguments::None;
        }
        let adt_generics = self.generics();
        // Generics for `#[implement(for<GENERICS> MyStruct<GENERICS>)]``
        let trait_ty_generics = input.trait_ty_generics();
        let trait_path: Path = if let Some(args) = &trait_ty_generics {
            parse_quote!(#path<#args>)
        } else {
            path.clone()
        };

        let mut correct_assoc_tys: CorrectAssocTys = Default::default();
        correct_assoc_tys.visit_item_trait(&input.trait_def);

        let where_clause = input
            .implementor
            .generics
            .as_ref()
            .and_then(|(_, g)| g.where_clause.clone())
            .map(|w| w.predicates.iter().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        let impl_generics = merge_generic_params(
            adt_generics.params.clone(),
            input
                .implementor
                .generics
                .as_ref()
                .map(|(_, g)| &g.params)
                .into_iter()
                .flatten()
                .cloned(),
        )
        .collect::<Punctuated<_, Token![,]>>();
        let pred_tys = self.get_predicate_types(&input.implementor);
        let mut impl_generics_modified = impl_generics.clone();
        let mut implr_args = trait_ty_generics.clone().unwrap_or_default();
        let items = input.trait_def.items.iter().map(|trait_item| match trait_item {
            TraitItem::Fn(tfn) => {
                let tokens = self.emit_trait_fn(
                    &trait_path,
                    &input.implementor,
                    tfn.clone(),
                    nonce,
                    leaked_ty_visitor
                );
                quote! {
                    #(if &tfn.sig.ident == "ne") {
                        #[allow(clippy::clippy::partialeq_ne_impl)]
                    }
                    #tokens
                }
            }
            TraitItem::Type(ttyp) => {
                let mut ttyp = ttyp.clone();
                leaked_ty_visitor.visit_trait_item_type_mut(&mut ttyp);
                correct_assoc_tys.0.remove(&ttyp.ident);
                let (impl_generics, ty_generics, where_clause) = ttyp.generics.split_for_impl();
                if pred_tys.len() != 1 {
                    if ttyp.generics.params.is_empty() && where_clause.is_none() {
                        let new_tp = Ident::new(&format!("ASSOC_{}_{}", &ttyp.ident,nonce),ttyp.ident.span());
                        let assoc_ty = &ttyp.ident;
                        implr_args.push(parse_quote! {#assoc_ty = #new_tp});
                        impl_generics_modified.push(parse_quote! {#new_tp});
                        quote! {
                            type #{&ttyp.ident} = #new_tp;
                        }
                    } else {
                        abort!(&input.implementor.path, "cannot implement this trait to enum"; note = ttyp.span() => "because the trait has associated types with generics");
                    }
                } else {
                    quote! {
                        type #{&ttyp.ident} #impl_generics = <#{&pred_tys[0]} as #trait_path>::#{&ttyp.ident} #ty_generics #where_clause;
                    }
                }
            }
            TraitItem::Const(tconst) => self.emit_trait_const(
                &trait_path,
                &input.implementor,
                tconst,
            ),
            o => abort!(o, "Not supported"),
        }).collect::<Vec<_>>();
        let detected_implicit_assoc_tys = correct_assoc_tys
            .0
            .iter()
            .enumerate()
            .map(|(n, name)| {
                (
                    name.clone(),
                    Ident::new(
                        &format!("IMPL_ASSOC_{}_{}", &name, nonce + n as u64),
                        Span::call_site(),
                    ),
                )
            })
            .collect::<Vec<_>>();
        let mut trait_supertraits = input.trait_def.supertraits.clone();
        for supertrait in trait_supertraits.iter_mut() {
            leaked_ty_visitor.visit_type_param_bound_mut(supertrait);
            if let TypeParamBound::Trait(TraitBound { path, .. }) = supertrait {
                if let Some(PathSegment { arguments, .. }) = path.segments.last_mut() {
                    if let PathArguments::None = &arguments {
                        *arguments =
                            PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                colon2_token: Default::default(),
                                lt_token: Default::default(),
                                args: Punctuated::new(),
                                gt_token: Default::default(),
                            });
                    }
                    if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args,
                        ..
                    }) = arguments
                    {
                        for (ident, par) in detected_implicit_assoc_tys.iter().cloned() {
                            args.push(GenericArgument::AssocType(AssocType {
                                ident,
                                generics: None,
                                eq_token: Token![=](Span::call_site()),
                                ty: parse_quote!(#par),
                            }));
                        }
                    }
                }
            }
        }
        impl_generics_modified.extend(
            detected_implicit_assoc_tys
                .iter()
                .map(|(_, par)| parse_quote! {#par})
                .collect::<Vec<GenericParam>>(),
        );
        let pred_bounds = quote! {#path <#(for arg in &implr_args){#arg,} #(for (name, par) in &detected_implicit_assoc_tys) {#name = #par,}>};
        quote! {
            #[automatically_derived]
            #{&input.trait_def.unsafety} impl < #impl_generics_modified > #trait_path for #{self.ident()} #{adt_generics.split_for_impl().1}
            where
                #(#where_clause,)*
                #(for st in &trait_supertraits) {
                    Self: #st,
                }
                #(#pred_tys: #pred_bounds),*
            {
                #(#items)*
            }
            #(if input.alternative.is_some()) {
                #[automatically_derived]
                unsafe impl < #impl_generics_modified > #{&input.implementor.path} for #{self.ident()} #{adt_generics.split_for_impl().1}
                where
                    #(#where_clause,)*
                    #(for st in &trait_supertraits) {
                        Self: #st,
                    }
                    #(#pred_tys: #pred_bounds),*
                {}
            }
        }
    }

    fn generics(&self) -> &Generics;

    fn ident(&self) -> &Ident;

    fn emit_body(
        &self,
        preds: &[(usize, Ident, TokenStream)],
        implementor: &Implementor,
        f: impl FnMut(&[Ident]) -> TokenStream,
    ) -> TokenStream;
}

impl EmitImpl for ItemEnum {
    fn emit_body(
        &self,
        preds: &[(usize, Ident, TokenStream)],
        implementor: &Implementor,
        mut f: impl FnMut(&[Ident]) -> TokenStream,
    ) -> TokenStream {
        let pred_param = Ident::new("__newer_type_pred_param", Span::call_site());
        quote! {
            match #{&preds[0].1} {
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
                        )} => {#{
                            let l = [pred_param.clone()];
                            f(&l[..])
                        }}
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

    fn get_predicate_types(&self, implementor: &Implementor) -> Vec<Type> {
        self.variants
            .iter()
            .map(|v| {
                find_pred_field(&implementor.clone(), &v.fields)
                    .1
                    .ty
                    .clone()
            })
            .collect()
    }
}

impl EmitImpl for ItemStruct {
    fn emit_body(
        &self,
        preds: &[(usize, Ident, TokenStream)],
        implementor: &Implementor,
        mut f: impl FnMut(&[Ident]) -> TokenStream,
    ) -> TokenStream {
        let pred_params = (0..preds.len())
            .map(|i| Ident::new(&format!("__newer_type_pred_param_{i}"), Span::call_site()))
            .collect::<Vec<_>>();
        let (n, pred_field) = find_pred_field(implementor, &self.fields);
        quote! {
            #(if let Fields::Named(_) = &self.fields) {
                #(for ((_, pred_ident, _), pred_param) in preds.iter().zip(&pred_params)) {
                    let Self {#{&pred_field.ident}: #pred_param, ..} = #pred_ident;
                }
            }
            #(if let Fields::Unnamed(_) = &self.fields) {
                #(for ((_, pred_ident, _), pred_param) in preds.iter().zip(&pred_params)) {
                    let Self (
                        #(for (i, _) in self.fields.iter().enumerate()), {
                            #(if i == n) {
                                #pred_param
                            }
                            #(else) { _ }
                        }
                    ) = #pred_ident;
                }
            }
            #{ f(&pred_params) }
        }
    }

    fn generics(&self) -> &Generics {
        &self.generics
    }

    fn ident(&self) -> &Ident {
        &self.ident
    }

    fn get_predicate_types(&self, implementor: &Implementor) -> Vec<Type> {
        vec![find_pred_field(implementor, &self.fields).1.ty.clone()]
    }
}

impl Input {
    fn modify_implr_generics(&mut self, nonce: u64) {
        if let Some(implr_generics) = &mut self.implementor.generics {
            let mut implr_modifier: ModifyGenerics = Default::default();
            for p in implr_generics.1.params.iter_mut() {
                match p {
                    GenericParam::Lifetime(lt) => {
                        implr_modifier.append_lifetime(&mut lt.lifetime, nonce)
                    }
                    GenericParam::Type(TypeParam { ident, .. }) => {
                        implr_modifier.append_type(ident, nonce)
                    }
                    GenericParam::Const(ConstParam { ident, .. }) => {
                        implr_modifier.append_const(ident, nonce)
                    }
                }
            }
            implr_modifier.visit_path_mut(&mut self.implementor.path);
        }
    }

    fn modify_adt_generics(&mut self, nonce: u64) {
        let adt_generics = self.adt.generics_mut();
        let mut type_modifier: ModifyGenerics = Default::default();
        for param in &mut adt_generics.params {
            match param {
                GenericParam::Lifetime(LifetimeParam { lifetime, .. }) => {
                    type_modifier.append_lifetime(lifetime, nonce + 1)
                }
                GenericParam::Type(TypeParam { ident, .. }) => {
                    type_modifier.append_type(ident, nonce + 1)
                }
                GenericParam::Const(ConstParam { ident, .. }) => {
                    type_modifier.append_const(ident, nonce + 1)
                }
            }
        }
        if let Some((_, implr_generics)) = &mut self.implementor.generics {
            type_modifier.visit_generics_mut(implr_generics);
        }
        type_modifier.visit_path_mut(&mut self.implementor.path);
        match &mut self.adt {
            Adt::Enum(item_enum) => type_modifier.visit_item_enum_mut(item_enum),

            Adt::Struct(item_struct) => type_modifier.visit_item_struct_mut(item_struct),
        }
    }

    fn modify_trait_generics(&mut self) {
        let mut trait_modifier: ModifyGenerics = Default::default();
        let implr_args = self.trait_ty_generics().unwrap_or_default();
        for (i, tr_arg) in self.trait_def.generics.params.iter().enumerate() {
            if let Some(implr_arg) = implr_args.get(i) {
                match (tr_arg, implr_arg) {
                    (
                        GenericParam::Lifetime(LifetimeParam { lifetime, .. }),
                        GenericArgument::Lifetime(lt1),
                    ) => {
                        trait_modifier
                            .lifetime_map
                            .insert(lifetime.clone(), lt1.clone());
                    }
                    (GenericParam::Type(TypeParam { ident, .. }), GenericArgument::Type(t1)) => {
                        trait_modifier.type_map.insert(ident.clone(), t1.clone());
                    }
                    (GenericParam::Const(ConstParam { ident, .. }), GenericArgument::Const(c1)) => {
                        trait_modifier.const_map.insert(ident.clone(), c1.clone());
                    }
                    _ => {
                        abort!(implr_arg, "cannot assign this argument"; hint = tr_arg.span() => "param definition is here")
                    }
                }
            } else {
                match tr_arg {
                    GenericParam::Type(TypeParam {
                        ident,
                        eq_token: Some(_),
                        default: Some(ty),
                        ..
                    }) => {
                        trait_modifier.type_map.insert(ident.clone(), ty.clone());
                    }
                    GenericParam::Const(ConstParam {
                        ident,
                        eq_token: Some(_),
                        default: Some(expr),
                        ..
                    }) => {
                        trait_modifier.const_map.insert(ident.clone(), expr.clone());
                    }
                    GenericParam::Type(TypeParam { ident, .. })
                    | GenericParam::Const(ConstParam { ident, .. }) => {
                        abort!(
                            &implr_args, "parameter '{}' is not specified for trait {}", ident, &self.trait_def.ident;
                            hint = ident.span() => "defined here";
                        );
                    }
                    GenericParam::Lifetime(LifetimeParam { lifetime, .. }) => {
                        abort!(
                            &implr_args, "lifetime parameter '{}' is not specified", &lifetime;
                            hint = lifetime.span() => "defined here";
                        );
                    }
                }
            }
        }
        if let Some((_, implr_generics)) = &mut self.implementor.generics {
            trait_modifier.visit_generics_mut(implr_generics);
        }
        for item in self.trait_def.items.iter_mut() {
            trait_modifier.visit_trait_item_mut(item);
        }
        for supertrait in self.trait_def.supertraits.iter_mut() {
            trait_modifier.visit_type_param_bound_mut(supertrait);
        }
    }

    fn make_leaked_ty_visitor(&self, nonce: u64) -> impl VisitMut {
        let encoded_generics =
            type_leak::encode_generics_to_ty(self.trait_ty_generics().iter().flatten());
        let repeater_path = self.repeater.clone();
        self.referrer.clone().into_visitor(move |_, id| {
            let repeater: Path = parse_quote!(#repeater_path<#nonce, #id, #encoded_generics>);
            parse_quote!(<Self as #repeater>::Type)
        })
    }

    fn trait_ty_generics(&self) -> Option<Punctuated<GenericArgument, Token![,]>> {
        match &self
            .implementor
            .path
            .segments
            .last()
            .unwrap_or_else(|| abort!(&self.implementor, "Path without segments is not supported"))
            .arguments
        {
            PathArguments::None => None,
            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
                Some(args.clone())
            }
            _ => abort!(&self.implementor, "bad generic arguments"),
        }
    }

    pub fn implement_internal(&self) -> TokenStream {
        let mut input = self.clone();
        let nonce = crate::random();
        input.modify_implr_generics(nonce);
        input.modify_adt_generics(nonce);
        input.modify_trait_generics();
        let mut leaked_ty_visitor = input.make_leaked_ty_visitor(self.nonce);
        match &input.adt {
            Adt::Enum(item_enum) => item_enum.emit_impl(&input, nonce, &mut leaked_ty_visitor),
            Adt::Struct(item_struct) => {
                item_struct.emit_impl(&input, nonce, &mut leaked_ty_visitor)
            }
        }
    }
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
            return (0, fields.iter().next().cloned().unwrap());
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
