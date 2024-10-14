use quote::quote;

mod attrs;
mod field;
mod item;

/// Parses the model.
pub fn parse(
    attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attrs = syn::parse_macro_input!(attrs as attrs::ModelAttrs);
    let mut input = syn::parse_macro_input!(input as item::Item);
    for (k, fr) in &attrs.refs {
        let fr = field::FieldRef::new(
            k.to_string(),
            fr.array,
            fr.target.clone(),
            fr.target_field.clone(),
            false,
        );
        input.fields.push_field_ref(fr, "id");
    }

    let col_name = &attrs.name;
    let ident = &input.ident;
    let graphql = if attrs.graphql {
        input.graphql(attrs.graphql_input, attrs.graphql_output)
    } else {
        quote! {}
    };

    quote! {
        #input
        const _: () = {
            impl ::mangga::Model for #ident {
                const COLLECTION: &'static str = #col_name;
            }
        };
        #graphql
    }
    .into()
}
