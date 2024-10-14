use crate::{AsUpdate, Executable, ManggaDoc, Model};

/// Operation for updating many documents
pub struct UpdateMany<M: ManggaDoc, U: AsUpdate> {
    filter: bson::Document,
    update: U,
    options: Option<mongodb::options::UpdateOptions>,
    _model: std::marker::PhantomData<M>,
}

impl<M: ManggaDoc, U: AsUpdate> UpdateMany<M, U> {
    /// Create a new `UpdateMany` operation
    pub fn new(filter: bson::Document, update: U) -> Self {
        Self {
            filter,
            update,
            options: None,
            _model: std::marker::PhantomData,
        }
    }

    /// Set the options
    pub fn options<F>(mut self, f: F) -> Self
    where
        F: FnOnce(
            mongodb::options::UpdateOptionsBuilder<((), (), (), (), (), (), (), ())>,
        ) -> mongodb::options::UpdateOptions,
    {
        self.options = Some(f(mongodb::options::UpdateOptions::builder()));
        self
    }
}

/// Operation for updating one document
pub struct UpdateOne<M: ManggaDoc, U: AsUpdate> {
    filter: bson::Document,
    update: U,
    options: Option<mongodb::options::UpdateOptions>,
    _model: std::marker::PhantomData<M>,
}

impl<M: ManggaDoc, U: AsUpdate> UpdateOne<M, U> {
    /// Create a new `UpdateOne` operation
    pub fn new(filter: bson::Document, update: U) -> Self {
        Self {
            filter,
            update,
            options: None,
            _model: std::marker::PhantomData,
        }
    }

    /// Set the options
    pub fn options<F>(mut self, f: F) -> Self
    where
        F: FnOnce(
            mongodb::options::UpdateOptionsBuilder<((), (), (), (), (), (), (), ())>,
        ) -> mongodb::options::UpdateOptions,
    {
        self.options = Some(f(mongodb::options::UpdateOptions::builder()));
        self
    }
}

impl<M: ManggaDoc, U: AsUpdate> Executable for UpdateMany<M, U> {
    type Model = M;
    type Output = ();

    async fn execute(self) -> crate::error::Result<Self::Output> {
        let col = M::Model::collection()?;
        if let Some(options) = self.options {
            col.update_many(self.filter, self.update.as_update()?)
                .with_options(options)
                .await?;
        } else {
            col.update_many(self.filter, self.update.as_update()?)
                .await?;
        }
        Ok(())
    }
}

impl<M: ManggaDoc, U: AsUpdate> Executable for UpdateOne<M, U> {
    type Model = M;
    type Output = ();

    async fn execute(self) -> crate::error::Result<Self::Output> {
        let col = M::Model::collection()?;
        if let Some(options) = self.options {
            col.update_one(self.filter, self.update.as_update()?)
                .with_options(options)
                .await?;
        } else {
            col.update_one(self.filter, self.update.as_update()?)
                .await?;
        }
        Ok(())
    }
}
