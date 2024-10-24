use crate::{db::get_database, traits::Model, types::BoxFut, Result};
use mongodb::options::{
    InsertManyOptions,
    InsertManyOptionsBuilder,
    InsertOneOptions,
    InsertOneOptionsBuilder,
};
use serde::Serialize;
use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

/// InsertOne
///
/// Represents the insert one operation
pub struct InsertOne<'a, M: Model> {
    opts: Option<InsertOneOptions>,
    data: &'a M,
}

impl<'a, M: Model> InsertOne<'a, M> {
    /// Create a new insert one operation
    pub fn new(data: &'a M) -> Self {
        Self {
            opts: None,
            data,
        }
    }

    /// Set insert one options
    pub fn opts<F>(mut self, f: F) -> Self
    where
        F: FnOnce(InsertOneOptionsBuilder<((), (), ())>) -> InsertOneOptions,
    {
        self.opts = Some(f(InsertOneOptions::builder()));
        self
    }
}

impl<M: Model> IntoFuture for InsertOne<'_, M>
where
    M: Serialize,
{
    type IntoFuture = InsertOneFuture;
    type Output = Result<()>;

    fn into_future(self) -> Self::IntoFuture {
        let data = self.data.clone();
        let opts = self.opts;
        let future = Box::pin(async move {
            let db = get_database(M::DB_NAME)?;
            let col = db.collection(M::MODEL_NAME);
            col.insert_one(data).with_options(opts).await?;

            Ok(())
        });

        InsertOneFuture(future)
    }
}

/// InsertOneFuture
///
/// Represents the executor of the insert one operation
pub struct InsertOneFuture(BoxFut<()>);

impl Future for InsertOneFuture {
    type Output = Result<()>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.get_mut().0.as_mut().poll(cx)
    }
}

/// InsertMany
///
/// Represents the insert many operation
pub struct InsertMany<M: Model> {
    data: Vec<M>,
    opts: Option<InsertManyOptions>,
}

impl<M: Model> InsertMany<M> {
    /// Create a new insert many operation
    pub fn new(data: Vec<M>) -> Self {
        Self {
            data,
            opts: None,
        }
    }

    /// Set insert many options
    pub fn opts<F>(mut self, f: F) -> Self
    where
        F: FnOnce(InsertManyOptionsBuilder<((), (), (), ())>) -> InsertManyOptions,
    {
        self.opts = Some(f(InsertManyOptions::builder()));
        self
    }
}

impl<M: Model> IntoFuture for InsertMany<M>
where
    M: Serialize,
{
    type IntoFuture = InsertManyFuture;
    type Output = Result<()>;

    fn into_future(self) -> Self::IntoFuture {
        let data = self.data;
        let opts = self.opts;
        let future = Box::pin(async move {
            let db = get_database(M::DB_NAME)?;
            let col = db.collection(M::MODEL_NAME);
            col.insert_many(data).with_options(opts).await?;

            Ok(())
        });

        InsertManyFuture(future)
    }
}

/// InsertManyFuture
///
/// Represents the executor of the insert many operation
pub struct InsertManyFuture(BoxFut<()>);

impl Future for InsertManyFuture {
    type Output = Result<()>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.get_mut().0.as_mut().poll(cx)
    }
}
