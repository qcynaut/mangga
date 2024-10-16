use std::collections::HashSet;
use syn::parse::Parse;

/// ItemAttrs
///
/// Represents the attributes of a struct
#[derive(Debug, Clone)]
pub struct ItemAttrs {
    pub name: String,
    pub db_name: Option<String>,
}

impl Parse for ItemAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut keys: HashSet<String> = HashSet::new();
        let mut name = String::new();
        let mut db_name = None;

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
                "db" => db_name = Some(input.parse::<syn::LitStr>()?.value()),
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

        Ok(ItemAttrs { name, db_name })
    }
}
