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
use type_leak::Referrer;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Input {
    pub implementor: Implementor,
    pub target_def: TargetDef,
    pub trait_def: ItemTrait,
    pub alternative: Option<Path>,
    pub newer_type: Path,
    pub referrer: Referrer,
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
        let nonce: LitInt = input.parse()?;
        let _ = input.parse::<Token![,]>();
        if input.is_empty() {
            Ok(Self {
                implementor,
                target_def,
                trait_def,
                alternative,
                newer_type,
                referrer,
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
            target_def: self.target_def.clone(),
        };
        tokens.extend(quote! {(#impl_output)});
        self.trait_def.to_tokens(tokens);
        <Token![,]>::default().to_tokens(tokens);
        self.alternative.to_tokens(tokens);
        <Token![,]>::default().to_tokens(tokens);
        self.referrer.to_tokens(tokens);
        <Token![,]>::default().to_tokens(tokens);
        tokens.extend(quote! {#{self.nonce}})
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

#[derive(Default, Clone)]
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

trait EmitImpl {
    fn emit_trait_const(
        &self,
        trait_: &TokenStream,
        implementor: &Implementor,
        input: &TraitItemConst,
    ) -> TokenStream {
        let ptyp = self.get_predicate_type(implementor).unwrap_or_else(|| {
            abort!(&implementor.path, "cannot implement this trait to enum"; note = input.span() => "because the trait has associated const");
        });
        quote! {
            const #{&input.ident} : #{&input.ty} = <#ptyp as #trait_>::#{&input.ident};
        }
    }
    fn emit_trait_ty(
        &self,
        trait_: &TokenStream,
        implementor: &Implementor,
        input: &TraitItemType,
    ) -> TokenStream {
        let ptyp = self.get_predicate_type(implementor).unwrap_or_else(|| {
            abort!(&implementor.path, "cannot implement this trait to enum"; note = input.span() => "because the trait has associated types");
        });
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        quote! {
            type #{&input.ident} #impl_generics = <#ptyp as #trait_>::#{&input.ident} #ty_generics #where_clause;
        }
    }
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
        // let (pred_ix, pred_ident, pred_ref_tokens) =
        // find_pred_param(&input.sig.inputs)
        let preds = find_pred_param(&input.sig.inputs);
        if preds.len() == 0 {
            abort!(&input.sig.inputs, "no `Self` type is found"; hint = "exact one `self` type is required in parameters");
        } else if preds.len() > 1 && self.get_predicate_type(implementor).is_none() {
            // self is enum
            abort!(&preds[1].1, "multiple `Self` type is not supported"; hint = preds[0].1.span() => "first `Self` type is here");
        }
        if let ReturnType::Type(_, ty) = &input.sig.output {
            if let Some(reason) = check_has_self_ty(ty.as_ref()) {
                abort!(reason, "`Self` type is not allowed in return position");
            }
        }
        let body = self.emit_body(&preds, implementor, |pred_params| {
            quote! {
                <_ as #trait_> :: #{&input.sig.ident} (
                    #(for (i, param) in input.sig.inputs.iter().enumerate()), {
                        #(if let Some((_, pred_param)) = preds.iter().zip(pred_params).filter(|((n, _, _), _)| &i == n).next()) {
                            #pred_param
                        }
                        #(else) {
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
        });
        quote! {
            #{&input.sig.constness}
            #{&input.sig.asyncness}
            #{&input.sig.unsafety}
            #{&input.sig.abi}
            fn #{&input.sig.ident} #impl_generics (
                #{&input.sig.inputs}
                #{&input.sig.variadic}
            ) #{&input.sig.output} #{where_clause} {
                #body
            }
        }
    }

    fn get_predicate_type(&self, implementor: &Implementor) -> Option<Type>;

    fn emit_impl(&self, input: &Input) -> TokenStream {
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
                GenericParam::Type(TypeParam { ident, .. }) => modifier.append_type(ident, nonce),
                GenericParam::Const(ConstParam { ident, .. }) => {
                    modifier.append_const(ident, nonce)
                }
            }
        }
        modifier.visit_path_arguments_mut(&mut trait_ty_generics);
        let mut modifier: ModifyGenerics = Default::default();
        if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
            &trait_ty_generics
        {
            for (i, tr_arg) in input.trait_def.generics.params.iter().enumerate() {
                if let Some(implr_arg) = args.get(i) {
                    match (tr_arg, implr_arg) {
                        (
                            GenericParam::Lifetime(LifetimeParam { lifetime, .. }),
                            GenericArgument::Lifetime(lt1),
                        ) => {
                            modifier.lifetime_map.insert(lifetime.clone(), lt1.clone());
                        }
                        (
                            GenericParam::Type(TypeParam { ident, .. }),
                            GenericArgument::Type(t1),
                        ) => {
                            modifier.type_map.insert(ident.clone(), t1.clone());
                        }
                        (
                            GenericParam::Const(ConstParam { ident, .. }),
                            GenericArgument::Const(c1),
                        ) => {
                            modifier.const_map.insert(ident.clone(), c1.clone());
                        }
                        _ => {
                            abort!(implr_arg, "cannot assign this argument"; hint = tr_arg.span() => "param definition is here")
                        }
                    }
                }
            }
        }

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
        let mut trait_items = input.trait_def.items.clone();
        for item in trait_items.iter_mut() {
            modifier.visit_trait_item_mut(item);
        }

        quote! {
            impl < #impl_generics > #path #trait_ty_generics for #{self.ident()} #type_ty_generics
            #where_clause
            {
                #(for item in &trait_items) {
                    #(if let TraitItem::Fn(tfn) = item) {
                        #{self.emit_trait_fn(&quote!(#path #trait_ty_generics), &input.implementor, tfn.clone(), nonce)}
                    }
                    #(if let TraitItem::Type(ttyp) = item) {
                        #{self.emit_trait_ty(&quote!(#path #trait_ty_generics), &input.implementor, ttyp)}
                    }
                    #(if let TraitItem::Const(tconst) = item) {
                        #{self.emit_trait_const(&quote!(#path #trait_ty_generics), &input.implementor, tconst)}
                    }
                }
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
                            f(&l)
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

    fn get_predicate_type(&self, _implementor: &Implementor) -> Option<Type> {
        None
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
            .map(|i| Ident::new(&format!("__newer_type_pred_param_{}", i), Span::call_site()))
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

    fn get_predicate_type(&self, implementor: &Implementor) -> Option<Type> {
        Some(find_pred_field(implementor, &self.fields).1.ty.clone())
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
