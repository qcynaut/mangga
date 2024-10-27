use std::collections::HashSet;
use syn::{parse::Parse, Ident};

/// ItemAttrs
///
/// Represents the attributes of a struct
#[derive(Debug, Clone)]
pub struct ItemAttrs {
    pub name: String,
    pub db_name: String,
}

impl Parse for ItemAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut keys: HashSet<String> = HashSet::new();
        let span = input.span();
        let mut name = String::new();
        let mut db_name = String::new();

        while !input.is_empty() {
            let id = input.parse::<syn::Ident>()?;
            input.parse::<syn::Token![=]>()?;
            let id_str = id.to_string();
            if keys.contains(&id_str) {
                return Err(syn::Error::new_spanned(
                    id,
                    format!("duplicate attribute `{}`", id_str),
                ));
            }

            keys.insert(id_str.clone());

            match &*id_str {
                "name" => name = input.parse::<syn::LitStr>()?.value(),
                "db" => db_name = input.parse::<syn::LitStr>()?.value(),
                _ => {
                    return Err(syn::Error::new_spanned(
                        id,
                        format!("unknown attribute `{}`", id_str),
                    ))
                }
            }

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        if name.is_empty() || db_name.is_empty() {
            return Err(syn::Error::new(
                span,
                "name and db attributes are required",
            ));
        }

        Ok(ItemAttrs { name, db_name })
    }
}

/// ItemGraphql
/// 
/// Represents the graphql attributes of a struct
#[derive(Debug, Clone)]
pub struct ItemGraphql {
    pub input: bool,
    pub output: bool,
    pub result: Ident
}

impl Parse for ItemGraphql {
    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut keys: HashSet<String> = HashSet::new();
        let mut input = true;
        let mut output = true;
        let mut result = None;

        while !stream.is_empty() {
            let id = stream.parse::<syn::Ident>()?;
            stream.parse::<syn::Token![=]>()?;
            let id_str = id.to_string();
            if keys.contains(&id_str) {
                return Err(syn::Error::new_spanned(
                    id,
                    format!("duplicate attribute `{}`", id_str),
                ));
            }

            keys.insert(id_str.clone());

            match &*id_str {
                "input" => input = stream.parse::<syn::LitBool>()?.value(),
                "output" => output = stream.parse::<syn::LitBool>()?.value(),
                "result" => result = Some(stream.parse::<Ident>()?),
                _ => {
                    return Err(syn::Error::new_spanned(
                        id,
                        format!("unknown attribute `{}`", id_str),
                    ))
                }
            }

            if !stream.is_empty() {
                stream.parse::<syn::Token![,]>()?;
            }
        }

        let result = result.unwrap_or_else(|| Ident::new("::mangga::Result", proc_macro2::Span::call_site()));

        Ok(ItemGraphql { input, output, result })
    }
}