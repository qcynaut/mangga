use quote::{quote, ToTokens};
use syn::{parse::Parse, Ident};

pub struct Joinable {
    ty: Vec<String>,
}

impl Parse for Joinable {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ty = vec![];

        while !input.is_empty() {
            let t = input.parse::<syn::Type>()?;
            let ident = t.to_token_stream().to_string();
            ty.push(ident);
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self { ty })
    }
}

impl Joinable {
    fn ty(&self, name: &str) -> syn::Type {
        let mut segments = syn::punctuated::Punctuated::new();
        segments.push(syn::PathSegment {
            ident: Ident::new(name, proc_macro2::Span::call_site()),
            arguments: syn::PathArguments::None,
        });
        syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments,
            },
        })
    }

    fn gen(&self, num: usize) -> proc_macro2::TokenStream {
        let mut impl_generics =
            syn::punctuated::Punctuated::<proc_macro2::TokenStream, syn::Token![,]>::new();
        impl_generics.push(quote! {T});
        impl_generics.push(quote! {For});
        impl_generics.push(quote! {From});
        impl_generics.push(quote! {JoinGeneric});
        let mut where_clause =
            syn::punctuated::Punctuated::<proc_macro2::TokenStream, syn::Token![,]>::new();
        where_clause.push(quote! {T: Executable<Model = From> + IntoQuery});
        where_clause.push(quote! {For: ManggaDoc});
        where_clause.push(quote! {From: ManggaDoc});
        where_clause.push(quote! {JoinGeneric: FieldJoinable<For, From>});
        let mut input_tupple =
            syn::punctuated::Punctuated::<proc_macro2::TokenStream, syn::Token![,]>::new();
        let mut output_tupple =
            syn::punctuated::Punctuated::<proc_macro2::TokenStream, syn::Token![,]>::new();

        for i in 0..num {
            if let Some(t) = self.ty.get(i) {
                let tt = format!("{}{}", t, t);
                let ty1 = self.ty(t);
                let ty2 = self.ty(&tt);
                impl_generics.push(quote! {#ty1});
                impl_generics.push(quote! {#ty2});
                where_clause.push(quote! {#ty1: Field + IsJoinOutputArray});
                where_clause.push(quote! {#ty2: for<'de> Deserialize<'de>});
                input_tupple.push(quote! {(#ty2, #ty1)});
                output_tupple.push(quote! {(#ty2, #ty1)});
            }
        }

        if num < self.ty.len() {
            output_tupple.push(quote! {(JoinGeneric::Output, JoinGeneric)});
        }

        for _ in num..self.ty.len() {
            input_tupple.push(quote! {()});
            output_tupple.push(quote! {()});
        }

        output_tupple.pop();
        quote! {
            impl<#impl_generics> Joinable<T, JoinGeneric, For> for JoinableExecutor<T, (#input_tupple)> where #where_clause {
                type From = From;
                type Tupple = (#output_tupple);

                fn join(self, _for: For, field: JoinGeneric) -> JoinableExecutor<T, Self::Tupple> {
                    let mut doc = self.doc;
                    doc.push(field.as_join().doc);
                    JoinableExecutor {
                        s: self.s,
                        doc,
                        _joins: std::marker::PhantomData,
                    }
                }
            }
        }
    }
}

impl ToTokens for Joinable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for len in 1..self.ty.len() {
            tokens.extend(self.gen(len));
        }
    }
}
