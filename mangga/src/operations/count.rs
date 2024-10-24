use crate::{db::get_database, traits::Model, types::BoxFut, Result};
use bson::Document;
use mongodb::options::{CountOptions, CountOptionsBuilder};
use std::future::{Future, IntoFuture};

/// Count
///
/// Represents the count operation
pub struct Count<M: Model> {
    filter: Document,
    opts: Option<CountOptions>,
    __marker: std::marker::PhantomData<M>,
}

impl<M: Model> Count<M> {
    /// Create a new count operation
    pub fn new(filter: Document) -> Self {
        Self {
            filter,
            opts: None,
            __marker: std::marker::PhantomData,
        }
    }

    /// Set count options
    pub fn opts<F>(mut self, f: F) -> Self
    where
        F: FnOnce(CountOptionsBuilder<((), (), (), (), (), (), (), ())>) -> CountOptions,
    {
        self.opts = Some(f(CountOptions::builder()));
        self
    }
}

impl<M: Model> IntoFuture for Count<M> {
    type IntoFuture = CountFuture;
    type Output = Result<usize>;

    fn into_future(self) -> Self::IntoFuture {
        let opts = self.opts;
        let filter = self.filter;
        CountFuture(Box::pin(async move {
            let db = get_database(M::DB_NAME)?;
            let col = db.collection::<M>(M::MODEL_NAME);
            let res = col.count_documents(filter).with_options(opts).await?;
            Ok(res as usize)
        }))
    }
}

/// CountFuture
///
/// Represents the executor of the count operation
pub struct CountFuture(BoxFut<usize>);

impl Future for CountFuture {
    type Output = Result<usize>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.get_mut().0.as_mut().poll(cx)
    }
}
