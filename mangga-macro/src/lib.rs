use proc_macro::TokenStream;

mod join_macro;
mod model_macro;

/// This macro is used to create a model.
///
/// # Example
/// ```no_run
/// use mangga::{model, ID};
/// use serde::{Deserialize, Serialize};
///
/// #[model("users", refs = {role: {target: book::doc, array: true, target_field: "user_id"}})]
/// #[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// pub struct User {
///     pub name: String,
///     pub age: u8,
///     #[model(ref = {target: role::doc, array: false})]
///     pub role_id: ID,
/// }
///
/// #[model("roles", refs = {users: {target: user::doc, array: true, target_field: "role_id"}})]
/// #[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// pub struct Role {
///     pub name: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn model(attrs: TokenStream, input: TokenStream) -> TokenStream {
    model_macro::parse(attrs, input)
}

/// This macro is used to implement `JoinResult` trait for the given type.
#[proc_macro]
pub fn join_result(input: TokenStream) -> TokenStream {
    join_macro::parse_join_result(input)
}

/// This macro is used to implement `Joinable` trait for the given type.
#[proc_macro]
pub fn joinable(input: TokenStream) -> TokenStream {
    join_macro::parse_joinable(input)
}
