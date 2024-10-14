use quote::quote;

mod join_result;
mod joinable;

/// Parses the join result.
pub fn parse_join_result(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as join_result::JoinResult);
    quote! {#input}.into()
}

/// Parses the joinable.
pub fn parse_joinable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as joinable::Joinable);
    quote! {#input}.into()
}
