use crate::{Executable, ManggaDoc, Model};

/// Operation for deleting many documents
pub struct DeleteMany<M: ManggaDoc> {
    filter: bson::Document,
    options: Option<mongodb::options::DeleteOptions>,
    _model: std::marker::PhantomData<M>,
}

impl<M: ManggaDoc> DeleteMany<M> {
    /// Create a new `DeleteMany` operation
    pub fn new(filter: bson::Document) -> Self {
        Self {
            filter,
            options: None,
            _model: std::marker::PhantomData,
        }
    }

    /// Set the options
    pub fn options<F>(mut self, f: F) -> Self
    where
        F: FnOnce(
            mongodb::options::DeleteOptionsBuilder<((), (), (), (), ())>,
        ) -> mongodb::options::DeleteOptions,
    {
        self.options = Some(f(mongodb::options::DeleteOptions::builder()));
        self
    }
}

/// Operation for deleting one document
pub struct DeleteOne<M: ManggaDoc> {
    filter: bson::Document,
    options: Option<mongodb::options::DeleteOptions>,
    _model: std::marker::PhantomData<M>,
}

impl<M: ManggaDoc> DeleteOne<M> {
    /// Create a new `DeleteOne` operation
    pub fn new(filter: bson::Document) -> Self {
        Self {
            filter,
            options: None,
            _model: std::marker::PhantomData,
        }
    }

    /// Set the options
    pub fn options<F>(mut self, f: F) -> Self
    where
        F: FnOnce(
            mongodb::options::DeleteOptionsBuilder<((), (), (), (), ())>,
        ) -> mongodb::options::DeleteOptions,
    {
        self.options = Some(f(mongodb::options::DeleteOptions::builder()));
        self
    }
}

impl<M: ManggaDoc> Executable for DeleteMany<M> {
    type Model = M;
    type Output = ();

    async fn execute(self) -> crate::error::Result<Self::Output> {
        let col = M::Model::collection()?;
        if let Some(options) = self.options {
            col.delete_many(self.filter).with_options(options).await?;
        } else {
            col.delete_many(self.filter).await?;
        }

        Ok(())
    }
}

impl<M: ManggaDoc> Executable for DeleteOne<M> {
    type Model = M;
    type Output = ();

    async fn execute(self) -> crate::error::Result<Self::Output> {
        let col = M::Model::collection()?;
        if let Some(options) = self.options {
            col.delete_one(self.filter).with_options(options).await?;
        } else {
            col.delete_one(self.filter).await?;
        }

        Ok(())
    }
}
