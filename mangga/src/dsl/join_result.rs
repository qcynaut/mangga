use super::IsJoinOutputArray;
use crate::{
    error::{Error, Result},
    Field,
};
use mangga_macro::join_result;
use serde::Deserialize;

/// IntoJoinResult
pub trait IntoJoinResult {
    type Output;
    fn into_join_result(doc: &bson::Document) -> Result<Self::Output>;
}

join_result!(A, (), (), (), ());
join_result!(A, B, (), (), ());
join_result!(A, B, C, (), ());
join_result!(A, B, C, D, ());
join_result!(A, B, C, D, E);
