use std::collections::HashMap;
use syn::{parse::Parse, Ident, LitStr, Type};

/// The attributes that are used to create a model.
///
/// # Example
/// ```no_run
/// use mangga::model;
///
/// #[model("users", refs = {role: (role_id) -> Role})]
/// ```
#[derive(Clone)]
pub struct ModelAttrs {
    pub name: String,
    pub graphql: bool,
    pub graphql_input: bool,
    pub graphql_output: bool,
    pub refs: HashMap<String, Ref>,
}

impl Parse for ModelAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<LitStr>()?.value();
        let mut refs = HashMap::new();
        let mut graphql = false;
        let mut graphql_input = true;
        let mut graphql_output = true;
        if !input.is_empty() {
            input.parse::<syn::Token![,]>()?;
        }

        while !input.is_empty() {
            let name = input.parse::<Ident>()?;
            match name.to_string().as_str() {
                "graphql" => {
                    graphql = true;
                    if input.peek(syn::Token![=]) {
                        input.parse::<syn::Token![=]>()?;
                        let curly;
                        syn::braced!(curly in input);

                        while !curly.is_empty() {
                            let ident = curly.parse::<Ident>()?;
                            match &*ident.to_string() {
                                "input" => {
                                    curly.parse::<syn::Token![:]>()?;
                                    graphql_input = curly.parse::<syn::LitBool>()?.value();
                                }
                                "output" => {
                                    curly.parse::<syn::Token![:]>()?;
                                    graphql_output = curly.parse::<syn::LitBool>()?.value();
                                }
                                _ => {
                                    return Err(syn::Error::new(
                                        ident.span(),
                                        "Invalid graphql attribute",
                                    ))
                                }
                            }

                            if !curly.is_empty() {
                                curly.parse::<syn::Token![,]>()?;
                            }
                        }
                    }
                }
                "refs" => {
                    let _ = input.parse::<syn::Token![=]>()?;
                    let curly;
                    syn::braced!(curly in input);
                    while !curly.is_empty() {
                        let name = curly.parse::<Ident>()?.to_string();
                        let _ = curly.parse::<syn::Token![:]>()?;
                        let mut array = false;
                        let mut target = None;
                        let mut target_field = String::from("id");
                        let curly2;
                        syn::braced!(curly2 in curly);

                        while !curly2.is_empty() {
                            let ident = curly2.parse::<Ident>()?;
                            match &*ident.to_string() {
                                "array" => {
                                    let _ = curly2.parse::<syn::Token![:]>()?;
                                    array = curly2.parse::<syn::LitBool>()?.value;
                                }
                                "target" => {
                                    let _ = curly2.parse::<syn::Token![:]>()?;
                                    target = Some(curly2.parse::<Type>()?);
                                }
                                "target_field" => {
                                    let _ = curly2.parse::<syn::Token![:]>()?;
                                    target_field = curly2.parse::<syn::LitStr>()?.value();
                                }
                                _ => {
                                    return Err(syn::Error::new(ident.span(), "Invalid field ref"))
                                }
                            }

                            if !curly2.is_empty() {
                                let _ = curly2.parse::<syn::Token![,]>()?;
                            }
                        }

                        if let Some(target) = target {
                            refs.insert(
                                name,
                                Ref {
                                    array,
                                    target,
                                    target_field,
                                },
                            );
                        } else {
                            return Err(syn::Error::new(
                                curly.span(),
                                "Missing target field in field ref",
                            ));
                        }

                        if !curly.is_empty() {
                            curly.parse::<syn::Token![,]>()?;
                        }
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        name.span(),
                        format!("unknown attribute `{}`", name.to_string()),
                    ));
                }
            }

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self {
            name,
            refs,
            graphql,
            graphql_input,
            graphql_output,
        })
    }
}

/// The attributes that are used to create a reference.
#[derive(Clone)]
pub struct Ref {
    pub array: bool,
    pub target: syn::Type,
    pub target_field: String,
}
