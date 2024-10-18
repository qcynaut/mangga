use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, Fields, Ident, Token, Visibility};

/// ItemField
///
/// Represents a field in a struct
#[derive(Debug, Clone)]
pub struct ItemField {
    pub ident: syn::Ident,
    pub ty: syn::Type,
    pub vis: syn::Visibility,
}

/// ItemFields
///
/// Represents the fields of a struct
#[derive(Debug, Clone)]
pub struct ItemFields {
    pub fields: Punctuated<ItemField, Token![,]>,
    pub id_field: ItemField,
}

impl ItemFields {
    /// Parse the fields of a struct
    pub fn parse(input: Fields) -> syn::Result<Self> {
        let span = input.span();
        let named_fields = match input {
            Fields::Named(fields) => fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "Only named fields are supported",
                ))
            }
        };

        let mut id_found = false;
        let mut id_field = None;
        let mut fields = Punctuated::<ItemField, Token![,]>::new();

        for field in named_fields {
            let ident = match &field.ident {
                Some(ident) => ident,
                None => return Err(syn::Error::new_spanned(field, "Field name is required")),
            };

            let item_field = ItemField {
                ident: ident.clone(),
                ty: field.ty.clone(),
                vis: field.vis.clone(),
            };

            // check if field is _id or has #[serde(rename = "_id")]
            if !id_found {
                if ident == "_id" {
                    id_found = true;
                } else {
                    let serde = field
                        .attrs
                        .iter()
                        .filter(|attr| attr.path().is_ident("serde"))
                        .collect::<Vec<_>>();
                    for attr in serde {
                        let list = match attr.meta.require_list() {
                            Ok(list) => list,
                            Err(_) => continue,
                        };

                        let list = list.tokens.to_string().replace(" ", "");
                        if list.contains("rename=\"_id\"") {
                            id_found = true;
                            id_field = Some(item_field.clone());
                            break;
                        }
                    }
                }
            }

            fields.push(item_field);
        }

        if !id_found || id_field.is_none() {
            return Err(syn::Error::new(
                span,
                "No field for `id` is found. Add `_id` field or use `#[serde(rename = \"_id\")]` \
                 attribute.",
            ));
        }

        Ok(Self {
            fields,
            id_field: id_field.unwrap(),
        })
    }

    /// Generate code to check type of id field
    pub fn gen_check_id(&self) -> TokenStream {
        let id_ty = &self.id_field.ty;
        quote! {
            is_id::<#id_ty>();
        }
    }

    /// Generate new function
    pub fn builtin(&self, vis: &Visibility, dsl_name: &Ident) -> TokenStream {
        let mut args = Punctuated::<TokenStream, Token![,]>::new();
        let mut names = Punctuated::<TokenStream, Token![,]>::new();
        let mut fields = quote! {};
        for field in &self.fields {
            let ident = &field.ident;
            let ty = &field.ty;
            args.push(quote! { #ident: impl Into<#ty> });
            names.push(quote! { #ident: #ident.into() });
            fields.extend(quote! {
                #[allow(non_upper_case_globals)]
                const #ident: #dsl_name::#ident = #dsl_name::#ident;
            });
        }
        quote! {
            #[allow(non_upper_case_globals)]
            const dsl: #dsl_name::dsl = #dsl_name::dsl;
            #fields
            #vis fn new(#args) -> Self {
                Self {
                    #names
                }
            }
        }
    }

    /// Generate field implementation
    pub fn impl_fields(&self, ident: &Ident) -> TokenStream {
        let fields = &self.fields;
        let mut code = quote! {};

        for field in fields {
            let field_ident = &field.ident;
            let field_ty = &field.ty;
            let field_name = field_ident.to_string();
            code.extend(quote! {
                #[derive(Debug, Clone, Copy)]
                pub struct #field_ident;
                impl Field for #field_ident {
                    type Model = #ident;
                    const NAME: &'static str = #field_name;
                    type Type = #field_ty;
                }
            });
        }

        code
    }
}
