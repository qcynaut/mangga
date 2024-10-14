use crate::{
    error::Result,
    traits::{ManggaDoc, Model},
    Executable,
    IntoQuery,
};
use futures::TryStreamExt;

/// Query for finding one document
pub struct FindOne<M: ManggaDoc> {
    filter: bson::Bson,
    options: Option<mongodb::options::FindOneOptions>,
    _model: std::marker::PhantomData<M>,
}

impl<M: ManggaDoc> FindOne<M> {
    /// Create a new `FindOne` query
    pub fn new(filter: bson::Bson) -> Self {
        Self {
            filter,
            options: None,
            _model: std::marker::PhantomData,
        }
    }

    /// Set the query options
    pub fn options<F>(mut self, f: F) -> Self
    where
        F: FnOnce(
            mongodb::options::FindOneOptionsBuilder<(
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
        ) -> mongodb::options::FindOneOptions,
    {
        self.options = Some(f(mongodb::options::FindOneOptions::builder()));
        self
    }
}

/// Query for finding many documents
pub struct FindMany<M: ManggaDoc> {
    filter: bson::Bson,
    options: Option<mongodb::options::FindOptions>,
    _model: std::marker::PhantomData<M>,
}

impl<M: ManggaDoc> FindMany<M> {
    /// Create a new `FindMany` query
    pub fn new(filter: bson::Bson) -> Self {
        Self {
            filter,
            options: None,
            _model: std::marker::PhantomData,
        }
    }

    /// Set the query options
    pub fn options<F>(mut self, f: F) -> Self
    where
        F: FnOnce(
            mongodb::options::FindOptionsBuilder<(
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
        ) -> mongodb::options::FindOptions,
    {
        self.options = Some(f(mongodb::options::FindOptions::builder()));
        self
    }
}

impl<M: ManggaDoc> Executable for FindOne<M> {
    type Model = M;
    type Output = Option<M::Model>;

    async fn execute(self) -> Result<Self::Output> {
        let col = M::Model::collection()?;
        let filter = self.filter.as_document().cloned().unwrap_or_default();
        let find = if let Some(options) = self.options {
            col.find_one(filter).with_options(options)
        } else {
            col.find_one(filter)
        };

        Ok(find.await?)
    }
}

impl<M: ManggaDoc> Executable for FindMany<M> {
    type Model = M;
    type Output = Vec<M::Model>;

    async fn execute(self) -> Result<Self::Output> {
        let col = M::Model::collection()?;
        let filter = self.filter.as_document().cloned().unwrap_or_default();
        let find = if let Some(options) = self.options {
            col.find(filter).with_options(options)
        } else {
            col.find(filter)
        };

        let cursor = find.await?;
        Ok(cursor.try_collect().await?)
    }
}

impl<M: ManggaDoc> IntoQuery for FindMany<M> {
    type Options = mongodb::options::FindOptions;

    fn filter(&self) -> bson::Document {
        self.filter.as_document().cloned().unwrap_or_default()
    }

    fn options(&self) -> Option<Self::Options> {
        self.options.clone()
    }
}

impl<M: ManggaDoc> IntoQuery for FindOne<M> {
    type Options = mongodb::options::FindOneOptions;

    fn filter(&self) -> bson::Document {
        self.filter.as_document().cloned().unwrap_or_default()
    }

    fn options(&self) -> Option<Self::Options> {
        self.options.clone()
    }
}
