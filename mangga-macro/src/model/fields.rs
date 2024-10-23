use change_case::upper_case;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, punctuated::Punctuated, spanned::Spanned, Fields, Ident, Token};

/// FieldIndex
///
/// Represents a field index
#[derive(Debug, Clone)]
pub struct FieldIndex {
    pub name: Option<String>,
    pub unique: bool,
    pub score: i32,
    pub exp: Option<u64>,
}

impl FieldIndex {
    /// Get token representation
    pub fn gen(&self, field: &Ident) -> TokenStream {
        let score = self.score;
        let unique = self.unique;
        let exp = &self.exp;
        let name = if let Some(name) = &self.name {
            name.to_owned()
        } else {
            let exp = if exp.is_some() { "exp" } else { "no-exp" };
            let unique = if unique { "unique" } else { "no-unique" };
            format!("mangga_index_{}_{}_{}_{}", field, score, unique, exp)
        };
        let field_str = field.to_string();
        let exp = if let Some(exp) = exp {
            quote! {Some(#exp)}
        } else {
            quote! {None}
        };

        quote! { (#field_str, #name, #score, #unique, #exp) }
    }
}

impl Parse for FieldIndex {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut unique = false;
        let mut score = 1;
        let mut exp = None;

        while !input.is_empty() {
            let id = input.parse::<syn::Ident>()?;
            let id_str = id.to_string();
            input.parse::<syn::Token![=]>()?;
            match &*id_str {
                "name" => name = Some(input.parse::<syn::LitStr>()?.value()),
                "unique" => unique = input.parse::<syn::LitBool>()?.value(),
                "exp" => exp = Some(input.parse::<syn::LitInt>()?.base10_parse()?),
                "score" => score = input.parse::<syn::LitInt>()?.base10_parse()?,
                _ => {
                    return Err(syn::Error::new_spanned(
                        id,
                        format!("unknown attribute `{}`", id_str),
                    ))
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(FieldIndex {
            name,
            unique,
            exp,
            score,
        })
    }
}

/// GraphqlRel
///
/// Represents the graphql relation of a field
#[derive(Debug, Clone)]
pub struct GraphqlRel {
    pub name: Ident,
    pub model: syn::Type,
    pub ty: String,
    pub field: Ident,
}

impl Parse for GraphqlRel {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let stream;
        syn::braced!(stream in input);

        let mut name = String::from("");
        let mut model = None;
        let mut ty = String::from("default");
        let mut field = Ident::new("id", span);
        let allowed_ty = vec!["array", "option", "default", "opt-array"];

        while !stream.is_empty() {
            let id = stream.parse::<syn::Ident>()?;
            stream.parse::<syn::Token![:]>()?;
            let id_str = id.to_string();
            match &*id_str {
                "name" => name = stream.parse::<syn::LitStr>()?.value(),
                "model" => model = Some(stream.parse::<syn::Type>()?),
                "ty" => {
                    let mty = stream.parse::<syn::LitStr>()?.value();
                    if !allowed_ty.contains(&mty.as_str()) {
                        return Err(syn::Error::new_spanned(
                            id,
                            format!("Allowed types are: {}", allowed_ty.join(", ")),
                        ));
                    }
                    ty = mty;
                },
                "field" => field = stream.parse::<Ident>()?,
                _ => {
                    return Err(syn::Error::new_spanned(
                        id,
                        format!("unknown attribute `{}`", id_str),
                    ))
                }
            }

            if !stream.is_empty() {
                stream.parse::<Token![,]>()?;
            }
        }

        if name.is_empty() {
            return Err(syn::Error::new(
                span,
                "name is required",
            ));
        }

        let name = Ident::new(&name, proc_macro2::Span::call_site());
        let model = model.ok_or_else(|| syn::Error::new(span, "model is required"))?;
        let field = Ident::new(&upper_case(&field.to_string()), field.span());

        Ok(GraphqlRel { name, model, ty, field })
    }
}

/// FieldGraphql
///
/// Represents the graphql attributes of a field
#[derive(Debug, Clone)]
pub struct FieldGraphql {
    pub input: bool,
    pub output: bool,
    pub rel: Option<GraphqlRel>,
}

impl Parse for FieldGraphql {
    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut input = true;
        let mut output = true;
        let mut rel = None;

        while !stream.is_empty() {
            let id = stream.parse::<syn::Ident>()?;
            stream.parse::<syn::Token![=]>()?;
            let id_str = id.to_string();
            match &*id_str {
                "input" => input = stream.parse::<syn::LitBool>()?.value(),
                "output" => output = stream.parse::<syn::LitBool>()?.value(),
                "rel" => rel = Some(stream.parse::<GraphqlRel>()?),
                _ => {
                    return Err(syn::Error::new_spanned(
                        id,
                        format!("unknown attribute `{}`", id_str),
                    ))
                }
            }

            if !stream.is_empty() {
                stream.parse::<Token![,]>()?;
            }
        }

        Ok(FieldGraphql { input, output, rel })
    }
}

/// FieldAttr
///
/// Represents a field attribute
#[derive(Debug, Clone)]
pub struct FieldAttr {
    pub indexes: Vec<FieldIndex>,
    pub graphql: FieldGraphql,
}

/// ItemField
///
/// Represents a field in a struct
#[derive(Debug, Clone)]
pub struct ItemField {
    pub name: String,
    pub ident: syn::Ident,
    pub ty: syn::Type,
    pub attrs: FieldAttr,
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

            let indexes = field
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("index"))
                .map(|attr| attr.parse_args_with(FieldIndex::parse))
                .collect::<syn::Result<Vec<_>>>()?;

            let graphql = field
                .attrs
                .clone()
                .into_iter()
                .filter(|attr| attr.path().is_ident("graphql"))
                .collect::<Vec<_>>();
            let mut graphql_tokens = Punctuated::<_, Token![,]>::new();

            for attr in graphql {
                let list = attr.meta.require_list()?;
                graphql_tokens.push(list.tokens.to_owned());
            }

            let graphql = syn::parse2(graphql_tokens.to_token_stream())?;
            let field_attr = FieldAttr { indexes, graphql };

            let mut item_field = ItemField {
                name: ident.to_string(),
                ident: ident.clone(),
                ty: field.ty.clone(),
                vis: field.vis.clone(),
                attrs: field_attr,
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
                            item_field.name = "_id".to_string();
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
}
