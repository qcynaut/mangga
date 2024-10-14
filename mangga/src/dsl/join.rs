use super::{
    join_result::IntoJoinResult,
    query::find::{FindMany, FindOne},
};
use crate::{
    error::{Error, Result},
    Executable,
    Field,
    IntoAggregate,
    IntoQuery,
    JoinExecutable,
    JoinExecutableSingle,
    ManggaDoc,
    Model,
};
use bson::Document;
use futures::TryStreamExt;
use mangga_macro::joinable;
use serde::Deserialize;

/// Join
pub struct Join<O> {
    doc: bson::Document,
    _output: std::marker::PhantomData<O>,
}

/// FieldJoinable
pub trait FieldJoinable<For: ManggaDoc, From: ManggaDoc>: Field + IsJoinOutputArray {
    type Output: for<'de> Deserialize<'de>;
    const IS_ARRAY: bool;
    const TARGET_FIELD: &'static str;
    /// Create a join
    fn as_join(self) -> Join<Self::Output> {
        let name = Self::NAME;
        let foreign_col = <For::Model as Model>::COLLECTION;
        Join {
            doc: bson::doc! {
                "$lookup": {
                    "from": foreign_col,
                    "localField": name,
                    "foreignField": Self::TARGET_FIELD,
                    "as": format!("{}_join", name),
                }
            },
            _output: std::marker::PhantomData,
        }
    }
}

/// Check if joinable field is array
pub trait IsJoinOutputArray {
    fn is_array() -> bool;
}

pub struct JoinableExecutor<T, J> {
    s: T,
    doc: Vec<bson::Document>,
    _joins: std::marker::PhantomData<J>,
}

/// Joinable
pub trait Joinable<T: Executable, J, For>: Sized {
    type From: ManggaDoc;
    type Tupple: IntoJoinResult;

    /// Join
    fn join(self, _for: For, field: J) -> JoinableExecutor<T, Self::Tupple>;
}

impl<T, From, J, For> Joinable<T, J, For> for T
where
    T: Executable<Model = From> + IntoQuery,
    From: ManggaDoc,
    J: FieldJoinable<For, From> + Field,
    For: ManggaDoc,
{
    type From = From;
    type Tupple = ((J::Output, J), (), (), (), ());

    fn join(self, _for: For, field: J) -> JoinableExecutor<T, Self::Tupple> {
        JoinableExecutor {
            s: self,
            doc: vec![field.as_join().doc],
            _joins: std::marker::PhantomData,
        }
    }
}

joinable!(A, B, C, D, E);

impl<M, J> JoinExecutable for JoinableExecutor<FindMany<M>, J>
where
    M: ManggaDoc,
    J: IntoJoinResult,
{
    type Output = Vec<(M::Model, J::Output)>;

    async fn execute(self) -> Result<Self::Output> {
        let filter = self.s.filter();
        let opts = IntoQuery::options(&self.s)
            .map(|o| o.pipeline())
            .unwrap_or_default();
        let mut lookups = vec![];
        lookups.push(bson::doc! { "$match": filter });
        lookups.extend(self.doc);
        lookups.extend(opts);
        let cursor = M::Model::collection()?.aggregate(lookups).await?;
        let res = cursor.try_collect::<Vec<Document>>().await?;
        let mut final_result = vec![];
        for doc in res {
            let primary: M::Model = bson::from_document(doc.clone())?;
            let secondary = J::into_join_result(&doc)?;
            final_result.push((primary, secondary));
        }
        Ok(final_result)
    }
}

impl<M, J> JoinExecutableSingle for JoinableExecutor<FindOne<M>, J>
where
    M: ManggaDoc,
    J: IntoJoinResult,
{
    type Output = (M::Model, J::Output);

    async fn execute(self) -> Result<Self::Output> {
        let filter = self.s.filter();
        let opts = IntoQuery::options(&self.s)
            .map(|o| o.pipeline())
            .unwrap_or_default();
        let mut lookups = vec![];
        lookups.push(bson::doc! { "$match": filter });
        lookups.extend(self.doc);
        lookups.extend(opts);
        let cursor = M::Model::collection()?.aggregate(lookups).await?;
        let res = cursor.try_collect::<Vec<Document>>().await?;
        let doc = res.first().ok_or(Error::DocumentNotFound)?;
        Ok((bson::from_document(doc.clone())?, J::into_join_result(doc)?))
    }
}
