use quote::{quote, ToTokens};
use syn::{parse::Parse, spanned::Spanned};

pub struct FieldRef {
    pub name: String,
    pub array: bool,
    pub target: syn::Type,
    pub target_field: String,
    pub optional: bool,
}

impl FieldRef {
    pub fn new(
        name: String,
        array: bool,
        target: syn::Type,
        target_field: String,
        optional: bool,
    ) -> Self {
        Self {
            name,
            array,
            target,
            target_field,
            optional,
        }
    }
}

impl Parse for FieldRef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let braced;
        syn::braced!(braced in input);
        let mut array = false;
        let mut target = None;
        let mut target_field = String::from("_id");
        let mut name = String::new();
        let mut optional = false;
        while !braced.is_empty() {
            let ident = braced.parse::<syn::Ident>()?;
            match &*ident.to_string() {
                "name" => {
                    braced.parse::<syn::Token![:]>()?;
                    name = braced.parse::<syn::LitStr>()?.value();
                }
                "array" => {
                    braced.parse::<syn::Token![:]>()?;
                    array = braced.parse::<syn::LitBool>()?.value();
                }
                "target" => {
                    braced.parse::<syn::Token![:]>()?;
                    target = Some(braced.parse::<syn::Type>()?);
                }
                "target_field" => {
                    braced.parse::<syn::Token![:]>()?;
                    target_field = braced.parse::<syn::LitStr>()?.value();
                }
                "optional" => {
                    optional = true;
                }
                _ => return Err(syn::Error::new(ident.span(), "Invalid field ref attribute")),
            }

            if !braced.is_empty() {
                let _ = braced.parse::<syn::Token![,]>()?;
            }
        }

        if let Some(target) = target {
            Ok(Self {
                name,
                array,
                target,
                target_field,
                optional,
            })
        } else {
            Err(syn::Error::new(
                input.span(),
                "Missing target field in field ref",
            ))
        }
    }
}

pub struct FieldIndex {
    pub name: String,
    pub score: i32,
    pub unique: bool,
}

impl Parse for FieldIndex {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let curly;
        syn::braced!(curly in input);
        let mut name = None;
        let mut score = 1;
        let mut unique = false;
        while !curly.is_empty() {
            let ident = curly.parse::<syn::Ident>()?;
            match &*ident.to_string() {
                "name" => {
                    let _ = curly.parse::<syn::Token![:]>()?;
                    name = Some(curly.parse::<syn::LitStr>()?.value());
                }
                "score" => {
                    let _ = curly.parse::<syn::Token![:]>()?;
                    score = curly.parse::<syn::LitInt>()?.base10_parse()?;
                }
                "unique" => {
                    let _ = curly.parse::<syn::Token![:]>()?;
                    unique = curly.parse::<syn::LitBool>()?.value;
                }
                _ => return Err(syn::Error::new(ident.span(), "Invalid field index")),
            }

            if !curly.is_empty() {
                let _ = curly.parse::<syn::Token![,]>()?;
            }
        }

        let name = if let Some(name) = name {
            name
        } else {
            String::from("")
        };

        Ok(Self {
            name,
            score,
            unique,
        })
    }
}

pub struct FieldGraphql {
    pub input: bool,
    pub input_type: Option<syn::Type>,
    pub output: bool,
}

impl Parse for FieldGraphql {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let braced;
        syn::braced!(braced in input);
        let mut input = true;
        let mut output = true;
        let mut input_type = None;
        while !braced.is_empty() {
            let ident = braced.parse::<syn::Ident>()?;
            match &*ident.to_string() {
                "input" => {
                    let _ = braced.parse::<syn::Token![:]>()?;
                    input = braced.parse::<syn::LitBool>()?.value;
                }
                "output" => {
                    let _ = braced.parse::<syn::Token![:]>()?;
                    output = braced.parse::<syn::LitBool>()?.value;
                }
                "input_type" => {
                    let _ = braced.parse::<syn::Token![:]>()?;
                    input_type = Some(braced.parse::<syn::Type>()?);
                }
                _ => return Err(syn::Error::new(ident.span(), "Invalid graphql attribute")),
            }
        }

        Ok(Self {
            input,
            output,
            input_type,
        })
    }
}

impl FieldGraphql {
    /// Create a new graphql field
    pub fn new() -> Self {
        Self {
            input: true,
            output: true,
            input_type: None,
        }
    }
}

pub struct Field {
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub ty: syn::Type,
    pub mutability: syn::FieldMutability,
    pub attrs: Vec<syn::Attribute>,
    pub field_ref: Vec<FieldRef>,
    pub field_index: Option<FieldIndex>,
    pub field_graphql: FieldGraphql,
}

impl Field {
    /// Parses the `syn::Field`.
    fn parse(field: syn::Field) -> syn::Result<Self> {
        let vis = field.clone().vis;
        let ident = if let Some(ident) = field.ident {
            ident
        } else {
            return Err(syn::Error::new(
                field.span(),
                "Only named fields are supported",
            ));
        };
        let ty = field.ty;
        let mutability = field.mutability;
        let mut attrs = vec![];
        let mut field_ref = vec![];
        let mut field_index = None;
        let mut field_graphql = FieldGraphql::new();

        for attr in field.attrs {
            if attr.path().is_ident("model") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("ref") {
                        let input = meta.input;
                        let _ = meta.input.parse::<syn::Token![=]>()?;
                        let mut r = FieldRef::parse(input)?;
                        if r.name.is_empty() {
                            r.name = ident.to_string();
                        }
                        field_ref.push(r);
                    } else if meta.path.is_ident("index") {
                        let input = meta.input;
                        let _ = meta.input.parse::<syn::Token![=]>()?;
                        let mut index = FieldIndex::parse(input)?;
                        let unique = if index.unique {
                            format!("_unique")
                        } else {
                            format!("")
                        };
                        if index.name.is_empty() {
                            index.name =
                                format!("mangga_field_{}_{}{}", &ident, index.score, unique);
                        } else {
                            index.name =
                                format!("mangga_field_{}_{}{}", &index.name, index.score, unique);
                        }

                        field_index = Some(index);
                    } else if meta.path.is_ident("graphql") {
                        let input = meta.input;
                        let _ = meta.input.parse::<syn::Token![=]>()?;
                        let graphql = FieldGraphql::parse(input)?;
                        field_graphql = graphql;
                    }

                    Ok(())
                })?;
            } else {
                attrs.push(attr);
            }
        }

        Ok(Self {
            vis,
            ident,
            ty,
            mutability,
            attrs,
            field_ref,
            field_index,
            field_graphql,
        })
    }

    /// Creates an id field
    pub fn id(vis: syn::Visibility) -> Self {
        Self {
            vis,
            ident: syn::Ident::new("id", proc_macro2::Span::call_site()),
            mutability: syn::FieldMutability::None,
            ty: syn::parse_quote!(::mangga::ID),
            attrs: vec![syn::parse_quote!(#[serde(rename = "_id")])],
            field_ref: vec![],
            field_index: None,
            field_graphql: FieldGraphql::new(),
        }
    }

    /// Synthesizes the `syn::Field`.
    fn syn(&self) -> syn::Field {
        syn::Field {
            attrs: self.attrs.clone(),
            vis: self.vis.clone(),
            ident: Some(self.ident.clone()),
            colon_token: Some(syn::Token![:](proc_macro2::Span::call_site())),
            ty: self.ty.clone(),
            mutability: self.mutability.clone(),
        }
    }

    /// Synthesizes the `syn::Field` as Option.
    fn syn_option(&self) -> syn::Field {
        let mut field = self.syn();
        let ty = field.ty.clone();
        field.ty = syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: syn::punctuated::Punctuated::from_iter(vec![syn::PathSegment {
                    ident: syn::Ident::new("Option", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: syn::token::Lt::default(),
                            args: syn::punctuated::Punctuated::from_iter(vec![
                                syn::GenericArgument::Type(ty),
                            ]),
                            gt_token: syn::token::Gt::default(),
                        },
                    ),
                }]),
            },
        });
        field
    }
}

pub struct Fields {
    pub fields: Vec<Field>,
}

impl Fields {
    /// Creates a new empty `Fields`.
    pub fn new() -> Self {
        Self { fields: vec![] }
    }

    /// Pushes an element to the back of the `Fields`.
    pub fn push(&mut self, value: syn::Field) -> syn::Result<()> {
        let field = Field::parse(value)?;
        self.fields.push(field);
        Ok(())
    }

    /// Pushes an parsed `Field` to the back of the `Fields`.
    pub fn push_field(&mut self, value: Field) {
        self.fields.push(value);
    }

    /// Pushes an `FieldRef` to the Field.
    pub fn push_field_ref(&mut self, value: FieldRef, field: &str) {
        if let Some(field) = self
            .fields
            .iter_mut()
            .find(|p| &*p.ident.to_string() == field)
        {
            field.field_ref.push(value);
        }
    }

    /// Get dsl tokens
    pub fn dsl(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let mut tokens = proc_macro2::TokenStream::new();

        for field in &self.fields {
            let name =
                syn::Ident::new(&format!("{}", &field.ident), proc_macro2::Span::call_site());
            let name_str = {
                let name = name.to_string();
                if &*name == "id" {
                    "_id".to_string()
                } else {
                    name
                }
            };
            let ty = &field.ty;

            tokens.extend(quote! {
                #[derive(Debug, Clone, Copy)]
                pub struct #name;
                impl ::mangga::Field for #name {
                    const NAME: &'static str = #name_str;
                    type Type = #ty;
                    type Doc = #ident;
                }
            });

            for field_ref in &field.field_ref {
                let target = &field_ref.target;
                let out_ty = quote! {<#target as ::mangga::ManggaDoc>::Model};
                let output = if field_ref.array {
                    quote! {Vec<#out_ty>}
                } else {
                    quote! {#out_ty}
                };
                let is_array = field_ref.array;
                let target_field = if &field_ref.target_field == "id" {
                    "_id"
                } else {
                    &field_ref.target_field
                };
                tokens.extend(quote! {
                    impl ::mangga::dsl::FieldJoinable<#target, super::doc> for #name {
                        type Output = #output;
                        const IS_ARRAY: bool = #is_array;
                        const TARGET_FIELD: &'static str = #target_field;
                    }
                    impl ::mangga::dsl::IsJoinOutputArray for #name {
                        fn is_array() -> bool {
                            Self::IS_ARRAY
                        }
                    }
                });
            }
        }

        quote! {
            pub mod fields {
                use super::*;
                #tokens
            }
        }
    }

    /// Get graphql input tokens
    pub fn graphql_input(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let input_ident =
            syn::Ident::new(&format!("{}Input", ident), proc_macro2::Span::call_site());
        let mut fields =
            syn::punctuated::Punctuated::<proc_macro2::TokenStream, syn::Token![,]>::new();
        let mut from_fields =
            syn::punctuated::Punctuated::<proc_macro2::TokenStream, syn::Token![;]>::new();

        for field in &self.fields {
            let name = &field.ident;
            if &*name.to_string() == "id" || !field.field_graphql.input {
                continue;
            }
            let ty = if let Some(ty) = &field.field_graphql.input_type {
                ty
            } else {
                &field.ty
            };
            let vis = &field.vis;
            fields.push(quote! {#vis #name: #ty});
            from_fields.push(quote! {val.#name = input.#name});
        }

        quote! {
            #[derive(Debug, Clone, Default, ::serde::Serialize, ::serde::Deserialize, ::async_graphql::InputObject)]
            pub struct #input_ident {
                #fields
            }

            impl From<#input_ident> for #ident {
                fn from(input: #input_ident) -> Self {
                    let mut val = Self::default();
                    #from_fields;
                    val
                }
            }
        }
    }

    /// Get graphql output tokens
    pub fn graphql_output(&self) -> proc_macro2::TokenStream {
        let mut tokens = proc_macro2::TokenStream::new();

        for field in &self.fields {
            if !field.field_graphql.output {
                continue;
            }
            let name = &field.ident;
            let ty = &field.ty;
            tokens.extend(quote! {
                async fn #name(&self) -> #ty {
                    self.#name.clone()
                }
            });

            for field_ref in &field.field_ref {
                let ref_name: syn::Ident =
                    syn::Ident::new(&field_ref.name, proc_macro2::Span::call_site());
                let ref_target = &field_ref.target;
                let ref_ty = if field_ref.array {
                    quote! {Vec<<#ref_target as ::mangga::ManggaDoc>::Model>}
                } else {
                    quote! {<#ref_target as ::mangga::ManggaDoc>::Model}
                };
                let ref_ty = if field_ref.optional {
                    quote! {Option<#ref_ty>}
                } else {
                    ref_ty
                };
                let ref_target_field = &field_ref.target_field;
                let inner = if field_ref.array {
                    if field_ref.optional {
                        quote! {
                            if let Some(val) = &self.#name {
                                Ok(Some(#ref_target::raw_filter(#ref_target_field, "$eq", val.clone()).find().execute().await?))
                            } else {
                                Ok(None)
                            }
                        }
                    } else {
                        quote! {
                            #ref_target::raw_filter(#ref_target_field, "$eq", self.#name.clone()).find().execute().await
                        }
                    }
                } else {
                    if field_ref.optional {
                        quote! {
                            if let Some(val) = &self.#name {
                                Ok(#ref_target::raw_filter(#ref_target_field, "$eq", val.clone()).find_one().execute().await?)
                            } else {
                                Ok(None)
                            }
                        }
                    } else {
                        quote! {
                            #ref_target::raw_filter(#ref_target_field, "$eq", self.#name.clone()).find_one().execute().await?.ok_or(::mangga::error::Error::DocumentNotFound)
                        }
                    }
                };
                tokens.extend(quote! {
                    async fn #ref_name(&self) -> ::mangga::error::Result<#ref_ty> {
                        #inner
                    }
                });
            }
        }

        tokens
    }

    /// Get impl new tokens
    pub fn impl_new(&self) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        let mut args = proc_macro2::TokenStream::new();
        let mut assignment = proc_macro2::TokenStream::new();
        for field in &self.fields {
            if !args.is_empty() {
                args.extend(quote! {, });
            }
            let name = &field.ident;
            let ty = &field.ty;
            if &*name.to_string() == "id" {
                assignment.extend(quote! {#name: Default::default(),});
            } else {
                args.extend(quote! {#name: impl Into<#ty>});
                assignment.extend(quote! {#name: #name.into(),});
            }
        }

        (args, assignment)
    }

    /// Get update tokens
    pub fn update(
        &self,
    ) -> (
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
    ) {
        let mut fields = syn::punctuated::Punctuated::<syn::Field, syn::Token![,]>::new();
        let mut assignments =
            syn::punctuated::Punctuated::<proc_macro2::TokenStream, syn::Token![,]>::new();
        let mut fns = proc_macro2::TokenStream::new();
        let mut match_fields = proc_macro2::TokenStream::new();
        let mut names =
            syn::punctuated::Punctuated::<proc_macro2::TokenStream, syn::Token![,]>::new();
        let mut from_impl = proc_macro2::TokenStream::new();

        for field in &self.fields {
            if &*field.ident.to_string() == "id" {
                continue;
            }
            fields.push(field.syn_option());
            let name = &field.ident;

            from_impl.extend(quote! {
                #name: Some(value.#name),
            });

            let name_str = &*name.to_string();
            names.push(quote! {
                #name_str
            });
            let ty = &field.ty;

            fns.extend(quote! {
                pub fn #name(mut self, #name: impl Into<#ty>) -> Self {
                    self.#name = Some(#name.into());
                    self
                }
            });

            assignments.push(quote! {
                #name: None
            });

            if match_fields.is_empty() {
                match_fields.extend(quote! {
                    if name == #name_str {
                        if let Some(val) = &self.#name {
                            ::mangga::bson::to_bson(val).ok()
                        } else {
                            None
                        }
                    }
                });
            } else {
                match_fields.extend(quote! {
                    else if name == #name_str {
                        if let Some(val) = &self.#name {
                            ::mangga::bson::to_bson(val).ok()
                        } else {
                            None
                        }
                    }
                });
            }
        }

        match_fields.extend(quote! {
            else {
                None
            }
        });

        fns.extend(quote! {
            pub fn new() -> Self {
                Self {
                    #assignments
                }
            }
        });

        (
            quote! {#fields},
            fns,
            match_fields,
            quote! {#names},
            from_impl,
        )
    }
}

impl ToTokens for Fields {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut punctuated = syn::punctuated::Punctuated::<syn::Field, syn::Token![,]>::new();

        for field in &self.fields {
            punctuated.push(field.syn());
        }

        let named_fields = syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: punctuated,
        };
        let fields = syn::Fields::Named(named_fields);

        fields.to_tokens(tokens);
    }
}
