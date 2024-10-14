use super::field::{Field, Fields};
use change_case::snake_case;
use quote::{quote, ToTokens};
use syn::{parse::Parse, spanned::Spanned};

pub struct Item {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub fields: Fields,
}

impl Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let struct_item = input.parse::<syn::ItemStruct>()?;
        let mut fields = Fields::new();

        fields.push_field(Field::id(struct_item.vis.clone()));

        match struct_item.fields {
            syn::Fields::Named(named) => {
                for field in named.named {
                    if let Some(ident) = &field.ident {
                        if &*ident.to_string() == "id" {
                            return Err(syn::Error::new(ident.span(), "The id field is reserved"));
                        }
                        fields.push(field)?;
                    }
                }
            }
            _ => {
                return Err(syn::Error::new(
                    struct_item.fields.span(),
                    "Only named fields are supported",
                ))
            }
        }

        Ok(Self {
            attrs: struct_item.attrs,
            vis: struct_item.vis,
            ident: struct_item.ident,
            generics: struct_item.generics,
            fields,
        })
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let attrs = &self.attrs;
        let vis = &self.vis;
        let ident = &self.ident;
        let generics = &self.generics;
        let fields = &self.fields;
        let (args, assignment) = self.fields.impl_new();
        let update = self.update();
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        tokens.extend(quote! {
            #(#attrs)*
            #vis struct #ident #generics #fields

            const _: () = {
                impl ::mangga::Identifiable for #ident {
                    fn mid(&self) -> &::mangga::ID {
                        &self.id
                    }
                }
                impl #impl_generics #ident #ty_generics #where_clause {
                    /// Creates a new instance of the model
                    pub fn new(#args) -> Self {
                        Self { #assignment }
                    }
                }
            };

            #update
        });
        tokens.extend(self.mods());
    }
}

impl Item {
    fn mods(&self) -> proc_macro2::TokenStream {
        let vis = &self.vis;
        let ident = &self.ident;
        let mod_ident = syn::Ident::new(&snake_case(&ident.to_string()), ident.span());
        let fields = self.fields.dsl(ident);
        let indexes = syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
            self.fields
                .fields
                .iter()
                .filter(|field| field.field_index.is_some())
                .map(|field| {
                    let ident = field.ident.to_string();
                    if let Some(index) = &field.field_index {
                        let name = &index.name;
                        let score = index.score;
                        let unique = index.unique;
                        Some(quote! {(#ident,#name,#score,#unique)})
                    } else {
                        None
                    }
                })
                .flatten(),
        );

        quote! {
            #[allow(unused_imports, non_camel_case_types, dead_code)]
            #vis mod #mod_ident {
                use super::*;
                use ::mangga::prelude::*;
                pub use self::fields::*;
                #[derive(Debug, Clone, Copy)]
                pub struct doc;
                impl ::mangga::ManggaDoc for doc {
                    type Model = #ident;
                    const INDEXES: &[(&'static str, &'static str, i32, bool)] = &[#indexes];
                }

                #fields
            }
        }
    }

    fn update(&self) -> proc_macro2::TokenStream {
        let vis = &self.vis;
        let ident = syn::Ident::new(
            &format!("{}Update", &self.ident),
            proc_macro2::Span::call_site(),
        );
        let from_ident = &self.ident;
        let generics = &self.generics;
        let (fields, fns, match_fields, names, from_impl) = self.fields.update();
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        quote! {
            #[derive(Debug, Clone, Default, ::serde::Serialize, ::serde::Deserialize)]
            #vis struct #ident #generics {
                #fields
            }
            const _: () = {
                impl #impl_generics #ident #ty_generics #where_clause {
                    #fns
                }

                impl #impl_generics ::mangga::AsUpdate for #ident #ty_generics #where_clause {
                    const FIELDS: &'static [&'static str] = &[#names];
                    fn field(&self, name: &str) -> Option<mangga::bson::Bson> {
                        #match_fields
                    }
                }

                impl #impl_generics From<#from_ident #ty_generics> for #ident #ty_generics #where_clause {
                    fn from(value: #from_ident #ty_generics) -> Self {
                        Self {
                            #from_impl
                        }
                    }
                }
            };
        }
    }

    pub fn graphql(&self, input: bool, output: bool) -> proc_macro2::TokenStream {
        let mut tokens = proc_macro2::TokenStream::new();
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        if input {
            tokens.extend(self.fields.graphql_input(ident));
        }
        if output {
            let tok = self.fields.graphql_output();
            tokens.extend(quote! {
                #[::async_graphql::Object]
                impl #impl_generics #ident #ty_generics #where_clause {
                    #tok
                }
            });
        }

        tokens
    }
}
