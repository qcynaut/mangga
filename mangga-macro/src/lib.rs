mod model;

#[proc_macro_derive(Model, attributes(mangga, index, graphql))]
pub fn model(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    model::parse(input)
}
