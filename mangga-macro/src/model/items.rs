use super::{attrs::ItemAttrs, fields::ItemFields};
use change_case::snake_case;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, Data, DeriveInput, Ident, Token};

/// Item
///
/// Represents a struct of mangga model
#[derive(Debug)]
pub struct Item {
    attrs: ItemAttrs,
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

        let struct_item = match input.data {
            Data::Struct(dt) => dt,
            _ => return Err(syn::Error::new_spanned(input, "Only structs are supported")),
        };

        let mut attr_tokens = Punctuated::<_, Token![,]>::new();
        for attr in &attrs {
            let list = attr.meta.require_list()?;
            attr_tokens.push(list.tokens.to_owned());
        }

        let attrs = syn::parse2::<ItemAttrs>(attr_tokens.to_token_stream())?;
        let fields = ItemFields::parse(struct_item.fields)?;

        Ok(Self {
            attrs,
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

    /// Generate dsl code
    pub fn dsl(&self) -> proc_macro2::TokenStream {
        let mut code = quote! {};

        let ident = &self.ident;
        let fields = self.fields.impl_fields(ident);
        let mod_ident = self.mod_ident();

        code.extend(quote! {
            #[allow(non_camel_case_types, dead_code)]
            mod #mod_ident {
                use super::*;
                #fields
                #[derive(Debug, Clone, Copy)]
                pub struct dsl;
                impl Dsl<#ident> for dsl {}
            }
        });

        code
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let vis = &self.vis;
        let ItemAttrs { name, db_name } = &self.attrs;

        // generate code
        let check_id = self.fields.gen_check_id();
        let dsl = self.dsl();
        let mod_ident = self.mod_ident();
        let builtin = self.fields.builtin(vis, &mod_ident);
        let id_field_ident = &self.fields.id_field.ident;

        let mut code = quote! {};

        code.extend(quote! {
            #check_id
            impl Model for #ident {
                const MODEL_NAME: &'static str = #name;
                const DB_NAME: &'static str = #db_name;
                fn id(&self) -> impl Into<ID> {
                    self.#id_field_ident
                }
                fn dsl() -> impl Dsl<Self> {
                    #mod_ident::dsl
                }
            }
            impl #ident {
                #builtin
            }
        });

        tokens.extend(quote! {
            const _: () = {
                #[allow(unused_imports)]
                use ::mangga::prelude::*;
                #dsl
                #code
            };
        });
    }
}
