use crate::{db::get_database, traits::Model, types::BoxFut, Result};
use bson::Document;
use mongodb::options::{DeleteOptions, DeleteOptionsBuilder};
use std::future::{Future, IntoFuture};

/// DeleteOne
///
/// Represents the delete one operation
pub struct DeleteOne<M: Model> {
    filter: Document,
    opts: Option<DeleteOptions>,
    __marker: std::marker::PhantomData<M>,
}

impl<M: Model> DeleteOne<M> {
    /// Create a new delete one operation
    pub fn new(filter: Document) -> Self {
        Self {
            filter,
            opts: None,
            __marker: std::marker::PhantomData,
        }
    }

    /// Set delete one options
    pub fn opts<F>(mut self, f: F) -> Self
    where
        F: FnOnce(DeleteOptionsBuilder<((), (), (), (), ())>) -> DeleteOptions,
    {
        self.opts = Some(f(DeleteOptions::builder()));
        self
    }
}

impl<M: Model> IntoFuture for DeleteOne<M> {
    type Output = Result<()>;
    type IntoFuture = DeleteOneFuture;

    fn into_future(self) -> Self::IntoFuture {
        let opts = self.opts;
        let filter = self.filter;
        DeleteOneFuture(Box::pin(async move {
            let db = get_database(M::DB_NAME)?;
            let col = db.collection::<M>(M::MODEL_NAME);
            col.delete_one(filter).with_options(opts).await?;
            Ok(())
        }))
    }
}

/// DeleteOneFuture
///
/// Represents the future of the delete one operation
pub struct DeleteOneFuture(BoxFut<()>);

impl Future for DeleteOneFuture {
    type Output = Result<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.get_mut().0.as_mut().poll(cx)
    }
}

/// DeleteMany
///
/// Represents the delete many operation
pub struct DeleteMany<M: Model> {
    filter: Document,
    opts: Option<DeleteOptions>,
    __marker: std::marker::PhantomData<M>,
}

impl<M: Model> DeleteMany<M> {
    /// Create a new delete many operation
    pub fn new(filter: Document) -> Self {
        Self {
            filter,
            opts: None,
            __marker: std::marker::PhantomData,
        }
    }

    /// Set delete many options
    pub fn opts<F>(mut self, f: F) -> Self
    where
        F: FnOnce(DeleteOptionsBuilder<((), (), (), (), ())>) -> DeleteOptions,
    {
        self.opts = Some(f(DeleteOptions::builder()));
        self
    }
}

impl<M: Model> IntoFuture for DeleteMany<M> {
    type Output = Result<()>;
    type IntoFuture = DeleteManyFuture;

    fn into_future(self) -> Self::IntoFuture {
        let opts = self.opts;
        let filter = self.filter;
        DeleteManyFuture(Box::pin(async move {
            let db = get_database(M::DB_NAME)?;
            let col = db.collection::<M>(M::MODEL_NAME);
            col.delete_many(filter).with_options(opts).await?;
            Ok(())
        }))
    }
}

/// DeleteManyFuture
///
/// Represents the future of the delete many operation
pub struct DeleteManyFuture(BoxFut<()>);

impl Future for DeleteManyFuture {
    type Output = Result<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.get_mut().0.as_mut().poll(cx)
    }
}
