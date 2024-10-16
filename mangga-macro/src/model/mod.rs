use quote::ToTokens;

mod attrs;
mod fields;
mod items;

pub fn parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match items::Item::parse(input) {
        Ok(item) => item.to_token_stream().into(),
        Err(e) => e.to_compile_error().into(),
    }
}
