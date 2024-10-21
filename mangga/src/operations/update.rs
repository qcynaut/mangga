use crate::{db::get_database, traits::Model, types::BoxFut, Result};
use bson::{doc, Bson, Document};
use mongodb::options::{UpdateOptions, UpdateOptionsBuilder};
use serde::Serialize;
use std::future::{Future, IntoFuture};

/// UpdateOne
///
/// Represents the update one operation
pub struct UpdateOne<M: Model> {
    opts: Option<UpdateOptions>,
    filter: Document,
    update: Document,
    __marker: std::marker::PhantomData<M>,
}

impl<M: Model> UpdateOne<M> {
    /// Create a new update one operation
    pub fn new(filter: Document, update: Vec<(String, Bson)>) -> Self {
        Self {
            opts: None,
            filter,
            update: doc! {
                "$set": Document::from_iter(update)
            },
            __marker: std::marker::PhantomData,
        }
    }

    /// Set update one options
    pub fn opts<F>(mut self, f: F) -> Self
    where
        F: FnOnce(UpdateOptionsBuilder<((), (), (), (), (), (), (), ())>) -> UpdateOptions,
    {
        self.opts = Some(f(UpdateOptions::builder()));
        self
    }
}

impl<M: Model> IntoFuture for UpdateOne<M>
where
    M: Serialize,
{
    type IntoFuture = UpdateOneFuture;
    type Output = Result<()>;

    fn into_future(self) -> Self::IntoFuture {
        let opts = self.opts;
        let filter = self.filter;
        let update = self.update;
        UpdateOneFuture(Box::pin(async move {
            let db = get_database(M::DB_NAME)?;
            let col = db.collection::<M>(M::MODEL_NAME);
            let _ = col.update_one(filter, update).with_options(opts).await?;
            Ok(())
        }))
    }
}

/// UpdateOneFuture
///
/// Represents the executor of the update one operation
pub struct UpdateOneFuture(BoxFut<()>);

impl Future for UpdateOneFuture {
    type Output = Result<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.get_mut().0.as_mut().poll(cx)
    }
}

/// UpdateMany
///
/// Represents the update many operation
pub struct UpdateMany<M: Model> {
    opts: Option<UpdateOptions>,
    filter: Document,
    update: Document,
    __marker: std::marker::PhantomData<M>,
}

impl<M: Model> UpdateMany<M> {
    /// Create a new update many operation
    pub fn new(filter: Document, update: Vec<(String, Bson)>) -> Self {
        Self {
            opts: None,
            filter,
            update: doc! {
                "$set": Document::from_iter(update)
            },
            __marker: std::marker::PhantomData,
        }
    }

    /// Set update many options
    pub fn opts<F>(mut self, f: F) -> Self
    where
        F: FnOnce(UpdateOptionsBuilder<((), (), (), (), (), (), (), ())>) -> UpdateOptions,
    {
        self.opts = Some(f(UpdateOptions::builder()));
        self
    }
}

impl<M: Model> IntoFuture for UpdateMany<M>
where
    M: Serialize,
{
    type IntoFuture = UpdateManyFuture;
    type Output = Result<()>;

    fn into_future(self) -> Self::IntoFuture {
        let opts = self.opts;
        let filter = self.filter;
        let update = self.update;
        UpdateManyFuture(Box::pin(async move {
            let db = get_database(M::DB_NAME)?;
            let col = db.collection::<M>(M::MODEL_NAME);
            let _ = col.update_many(filter, update).with_options(opts).await?;
            Ok(())
        }))
    }
}

/// UpdateManyFuture
///
/// Represents the executor of the update many operation
pub struct UpdateManyFuture(BoxFut<()>);

impl Future for UpdateManyFuture {
    type Output = Result<()>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.get_mut().0.as_mut().poll(cx)
    }
}