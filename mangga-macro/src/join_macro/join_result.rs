use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{parse::Parse, Type};

pub struct JoinResult {
    ty: Vec<String>,
}

impl Parse for JoinResult {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ty = vec![];

        while !input.is_empty() {
            let t = input.parse::<Type>()?;
            let ident = t.to_token_stream().to_string();
            ty.push(ident);
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self { ty })
    }
}

impl ToTokens for JoinResult {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut impl_generics =
            syn::punctuated::Punctuated::<proc_macro2::TokenStream, syn::Token![,]>::new();
        let mut where_clause =
            syn::punctuated::Punctuated::<proc_macro2::TokenStream, syn::Token![,]>::new();
        let mut item =
            syn::punctuated::Punctuated::<proc_macro2::TokenStream, syn::Token![,]>::new();
        let mut output = vec![];
        let mut inner_fn = HashMap::new();
        for ty in &self.ty {
            if ty == "()" {
                item.push(quote::quote! { () });
                continue;
            }
            let ident1 = syn::Ident::new(ty, proc_macro2::Span::call_site());
            let ident2 = syn::Ident::new(&format!("{}{}", ty, ty), proc_macro2::Span::call_site());
            inner_fn.insert(
                format!("{}", ident1).to_ascii_lowercase(),
                quote! {
                    let field_name = format!("{}_join", #ident1::NAME);
                    let doc = doc.get(field_name).ok_or(Error::DocumentNotFound)?.clone();
                    match doc {
                        bson::Bson::Array(arr) => {
                            if #ident1::is_array() {
                                Ok(bson::from_bson(bson::Bson::Array(arr))?)
                            } else {
                                let first = arr.first().cloned().ok_or(Error::DocumentNotFound)?;
                                Ok(bson::from_bson(first)?)
                            }
                        }
                        bson::Bson::Document(doc) => {
                            if #ident1::is_array() {
                                Ok(bson::from_bson(bson::Bson::Array(vec![
                                    bson::Bson::Document(doc),
                                ]))?)
                            } else {
                                Ok(bson::from_bson(bson::Bson::Document(doc))?)
                            }
                        }
                        _ => Err(Error::DocumentNotFound),
                    }
                },
            );
            output.push(ident2.clone());
            item.push(quote! {(#ident2, #ident1)});
            impl_generics.push(quote! {#ident1});
            impl_generics.push(quote! {#ident2});
            where_clause.push(quote! {#ident1: Field + IsJoinOutputArray});
            where_clause.push(quote! {#ident2: for<'de> Deserialize<'de>});
        }

        let output_ = if output.len() == 1 {
            let o = output[0].clone();
            quote! {#o}
        } else {
            let mut o = syn::punctuated::Punctuated::<syn::Ident, syn::Token![,]>::new();
            for item in output {
                o.push(item.clone());
            }

            quote! {(#o)}
        };
        let inner = if inner_fn.len() == 1 {
            let k = inner_fn.keys().next().cloned().unwrap_or_default();
            let i = inner_fn.get(&k).unwrap();
            let ident = syn::Ident::new(&k, proc_macro2::Span::call_site());
            quote! {
                let #ident = || {
                    #i
                };

                Ok(#ident()?)
            }
        } else {
            let mut ii = quote! {};
            let mut r =
                syn::punctuated::Punctuated::<proc_macro2::TokenStream, syn::Token![,]>::new();
            for (k, i) in inner_fn {
                let ident = syn::Ident::new(&k, proc_macro2::Span::call_site());
                ii.extend(quote! {
                    let #ident = || {
                        #i
                    };
                });
                r.push(quote! {#ident()?});
            }
            quote! {
                #ii
                Ok((#r))
            }
        };
        tokens.extend(quote! {
            impl<#impl_generics> IntoJoinResult for (#item) where #where_clause {
                type Output = #output_;

                fn into_join_result(doc: &bson::Document) -> Result<Self::Output> {
                    #inner
                }
            }
        })
    }
}
