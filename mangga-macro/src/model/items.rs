use super::{
    attrs::{ItemAttrs, ItemGraphql},
    fields::ItemFields,
};
use change_case::{snake_case, upper_case};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, Data, DeriveInput, Ident, Token};

/// Item
///
/// Represents a struct of mangga model
#[derive(Debug)]
pub struct Item {
    attrs: ItemAttrs,
    graphql_attrs: ItemGraphql,
    ident: syn::Ident,
    vis: syn::Visibility,
    fields: ItemFields,
}

impl Item {
    /// Parse the derive input
    pub fn parse(input: DeriveInput) -> syn::Result<Self> {
        // since we doesn't support generics, we will check for generics and return an error
        if input.generics.params.len() > 0 {
            return Err(syn::Error::new_spanned(
                input.generics,
                "Generics are not supported",
            ));
        }

        // filter mangga attributes
        let attrs = input
            .attrs
            .clone()
            .into_iter()
            .filter(|attr| attr.path().is_ident("mangga"))
            .collect::<Vec<_>>();

        // filter graphql attributes
        let graphql_attrs = input
            .attrs
            .clone()
            .into_iter()
            .filter(|attr| attr.path().is_ident("graphql"))
            .collect::<Vec<_>>();

        let struct_item = match input.data {
            Data::Struct(dt) => dt,
            _ => return Err(syn::Error::new_spanned(input, "Only structs are supported")),
        };

        let mut attr_tokens = Punctuated::<_, Token![,]>::new();
        for attr in &attrs {
            let list = attr.meta.require_list()?;
            attr_tokens.push(list.tokens.to_owned());
        }

        let mut graphql_attr_tokens = Punctuated::<_, Token![,]>::new();
        for attr in &graphql_attrs {
            let list = attr.meta.require_list()?;
            graphql_attr_tokens.push(list.tokens.to_owned());
        }

        let attrs = syn::parse2::<ItemAttrs>(attr_tokens.to_token_stream())?;
        let graphql_attrs = if graphql_attr_tokens.is_empty() {
            ItemGraphql {
                input: false,
                output: false,
                result: quote! {::mangga::Result},
            }
        } else {
            syn::parse2::<ItemGraphql>(graphql_attr_tokens.to_token_stream())?
        };
        let fields = ItemFields::parse(struct_item.fields)?;

        Ok(Self {
            attrs,
            graphql_attrs,
            ident: input.ident,
            vis: input.vis,
            fields,
        })
    }

    /// Get mod ident
    pub fn mod_ident(&self) -> Ident {
        let ident = &self.ident;
        let mod_ident = Ident::new(&snake_case(&ident.to_string()), ident.span());
        mod_ident
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let vis = &self.vis;
        let ItemAttrs { name, db_name } = &self.attrs;

        // generate code
        let check_id = self.fields.gen_check_id();
        let mut dsl = quote! {};
        let mod_ident = self.mod_ident();
        let id_field_ident = &self.fields.id_field.ident;
        let graphql_input_ident = Ident::new(&format!("{}Input", ident), ident.span());
        let graphql_res = &self.graphql_attrs.result;

        // builtin
        let mut builtin_args = Punctuated::<TokenStream, Token![,]>::new();
        let mut builtin_names = Punctuated::<TokenStream, Token![,]>::new();
        let mut indexes = Punctuated::<TokenStream, Token![,]>::new();
        let mut graphql_input_fields = Punctuated::<TokenStream, Token![,]>::new();
        let mut graphql_output = quote! {};
        let mut fields = quote! {};

        for field in &self.fields.fields {
            let field_vis = &field.vis;
            let field_ident = &field.ident;
            let field_ty = &field.ty;
            let field_name = &field.name;
            let const_field_ident =
                Ident::new(&upper_case(&field_ident.to_string()), field_ident.span());

            // generate dsl
            dsl.extend(quote! {
                #[derive(Debug, Clone, Copy)]
                pub struct #field_ident;
                impl Field for #field_ident {
                    type Model = #ident;
                    const NAME: &'static str = #field_name;
                    type Type = #field_ty;
                }
            });

            // builtin
            builtin_args.push(quote! { #field_ident: impl Into<#field_ty> });
            builtin_names.push(quote! { #field_ident: #field_ident.into() });
            fields.extend(quote! {
                #vis const #const_field_ident: #mod_ident::#field_ident = #mod_ident::#field_ident;
            });

            for index in &field.attrs.indexes {
                let token = index.gen(&field.ident);
                indexes.push(token);
            }

            // graphql
            if field.attrs.graphql.input {
                graphql_input_fields.push(quote! {
                    #field_vis #field_ident: #field_ty
                });
            }

            if field.attrs.graphql.output {
                graphql_output.extend(quote! {
                    async fn #field_ident(&self) -> #field_ty {
                        self.#field_ident.clone()
                    }
                });

                if let Some(rel) = field.attrs.graphql.rel.clone() {
                    let rel_model = rel.model;
                    let rel_field = rel.field;
                    let rel_name = rel.name;
                    let (rel_ty, inner) = match &*rel.ty {
                        "array" => {
                            let ty = quote! { Vec<#rel_model> };
                            let inner = quote! {
                                #rel_model::dsl.find_many(#rel_model::#rel_field.eq(self.#field_ident.clone())).await
                            };
                            (ty, inner)
                        }
                        "option" => {
                            let ty = quote! { Option<#rel_model> };
                            let inner = quote! {
                                if let Some(id) = &self.#field_ident {
                                    #rel_model::dsl.find_many(#rel_model::#rel_field.eq(id.clone())).await
                                } else {
                                    Ok(None)
                                }
                            };
                            (ty, inner)
                        }
                        "opt-array" => {
                            let ty = quote! { Option<Vec<#rel_model>> };
                            let inner = quote! {
                                if let Some(id) = &self.#field_ident {
                                    #rel_model::dsl.find_many(#rel_model::#rel_field.eq(id.clone())).await
                                } else {
                                    Ok(None)
                                }
                            };
                            (ty, inner)
                        }
                        _ => {
                            let ty = quote! { #rel_model };
                            let inner = quote! {
                                #rel_model::dsl.find_one(#rel_model::#rel_field.eq(self.#field_ident.clone())).await
                            };
                            (ty, inner)
                        }
                    };
                    if let Some(check_fn) = rel.check_fn {
                        graphql_output.extend(quote! {
                            async fn #rel_name(&self, ctx: &::async_graphql::Context<'_>) -> #graphql_res<#rel_ty> {
                                #check_fn(ctx).await?;
                                #inner.map_err(Into::into)
                            }
                        });
                    } else {
                        graphql_output.extend(quote! {
                            async fn #rel_name(&self) -> #graphql_res<#rel_ty> {
                                #inner.map_err(Into::into)
                            }
                        });
                    }
                }
            }
        }

        tokens.extend(
            quote! {
                const _: () = {
                    #[allow(unused_imports)]
                    use ::mangga::prelude::*;
                    #[allow(non_camel_case_types, dead_code)]
                    mod #mod_ident {
                        use super::*;
                        #dsl
                        #[derive(Debug, Clone, Copy)]
                        pub struct dsl;
                        impl Dsl<#ident> for dsl {}
                    }
                    #check_id
                    impl Model for #ident {
                        const MODEL_NAME: &'static str = #name;
                        const DB_NAME: &'static str = #db_name;
                        const INDEXES: &'static [(&'static str, &'static str, i32, bool, Option<u64>)] = &[#indexes];
                        fn id(&self) -> impl Into<ID> {
                            self.#id_field_ident
                        }
                    }
                    impl #ident {
                        #[allow(non_upper_case_globals)]
                        #vis const dsl: #mod_ident::dsl = #mod_ident::dsl;
                        #fields
                        #vis fn new(#builtin_args) -> Self {
                            Self {
                                #builtin_names
                            }
                        }
                    }
                };
            }
        );

        if self.graphql_attrs.input {
            tokens.extend(quote! {
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, ::async_graphql::InputObject)]
                #vis struct #graphql_input_ident {
                    #graphql_input_fields
                }
            });
        }

        if self.graphql_attrs.output {
            tokens.extend(quote! {
                #[::async_graphql::Object]
                impl #ident {
                    #graphql_output
                }
            });
        }
    }
}
