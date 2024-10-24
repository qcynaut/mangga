use crate::{db::get_database, traits::Model, types::BoxFut, Error, Result};
use bson::Document;
use futures::TryStreamExt;
use mongodb::options::{FindOneOptions, FindOneOptionsBuilder, FindOptions, FindOptionsBuilder};
use serde::Deserialize;
use std::future::{Future, IntoFuture};

/// FindOne
///
/// Represents the find one operation
pub struct FindOne<M: Model> {
    filter: Document,
    opts: Option<FindOneOptions>,
    __marker: std::marker::PhantomData<M>,
}

impl<M: Model> FindOne<M> {
    /// Create a new find one operation
    pub fn new(filter: Document) -> Self {
        Self {
            filter,
            opts: None,
            __marker: std::marker::PhantomData,
        }
    }

    /// Set find one options
    pub fn opts<F>(mut self, f: F) -> Self
    where
        F: FnOnce(
            FindOneOptionsBuilder<(
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            )>,
        ) -> FindOneOptions,
    {
        self.opts = Some(f(FindOneOptions::builder()));
        self
    }
}

impl<M: Model> FindOne<M>
where
    M: for<'de> Deserialize<'de>,
{
    /// Get optional result
    pub fn optional(self) -> BoxFut<Option<M>> {
        let opts = self.opts;
        let filter = self.filter;
        Box::pin(async move {
            let db = get_database(M::DB_NAME)?;
            let col = db.collection(M::MODEL_NAME);
            let res = col.find_one(filter).with_options(opts).await?;
            Ok(res)
        })
    }
}

impl<M: Model> IntoFuture for FindOne<M>
where
    M: for<'de> Deserialize<'de>,
{
    type IntoFuture = FindOneFuture<M>;
    type Output = Result<M>;

    fn into_future(self) -> Self::IntoFuture {
        let opts = self.opts;
        let filter = self.filter;
        FindOneFuture(Box::pin(async move {
            let db = get_database(M::DB_NAME)?;
            let col = db.collection(M::MODEL_NAME);
            let res = col.find_one(filter).with_options(opts).await?;
            if let Some(res) = res {
                Ok(res)
            } else {
                Err(Error::NotFound)
            }
        }))
    }
}

/// FindOneFuture
///
/// Represents the future of the find one operation
pub struct FindOneFuture<M: Model>(BoxFut<M>);

impl<M: Model> Future for FindOneFuture<M> {
    type Output = Result<M>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.get_mut().0.as_mut().poll(cx)
    }
}

/// FindMany
///
/// Represents the find many operation
pub struct FindMany<M: Model> {
    filter: Document,
    opts: Option<FindOptions>,
    __marker: std::marker::PhantomData<M>,
}

impl<M: Model> FindMany<M> {
    /// Create a new find many operation
    pub fn new(filter: Document) -> Self {
        Self {
            filter,
            opts: None,
            __marker: std::marker::PhantomData,
        }
    }

    /// Set find many options
    pub fn opts<F>(mut self, f: F) -> Self
    where
        F: FnOnce(
            FindOptionsBuilder<(
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            )>,
        ) -> FindOptions,
    {
        self.opts = Some(f(FindOptions::builder()));
        self
    }
}

impl<M: Model> IntoFuture for FindMany<M>
where
    M: for<'de> Deserialize<'de>,
{
    type IntoFuture = FindManyFuture<M>;
    type Output = Result<Vec<M>>;

    fn into_future(self) -> Self::IntoFuture {
        let opts = self.opts;
        let filter = self.filter;
        FindManyFuture(Box::pin(async move {
            let db = get_database(M::DB_NAME)?;
            let col = db.collection(M::MODEL_NAME);
            let res = col.find(filter).with_options(opts).await?;
            let res = res.try_collect::<Vec<_>>().await?;
            Ok(res)
        }))
    }
}

/// FindManyFuture
///
/// Represents the future of the find many operation
pub struct FindManyFuture<M: Model>(BoxFut<Vec<M>>);

impl<M: Model> Future for FindManyFuture<M> {
    type Output = Result<Vec<M>>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.get_mut().0.as_mut().poll(cx)
    }
}