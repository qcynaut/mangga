use crate::{error::Result, Executable, ManggaDoc, Model};

/// Operation for inserting many documents
pub struct InsertMany<M: ManggaDoc> {
    docs: Vec<M::Model>,
    options: Option<mongodb::options::InsertManyOptions>,
    _model: std::marker::PhantomData<M>,
}

impl<M: ManggaDoc> InsertMany<M> {
    /// Create a new `InsertMany` operation
    pub fn new(docs: Vec<M::Model>) -> Self {
        Self {
            docs,
            options: None,
            _model: std::marker::PhantomData,
        }
    }

    /// Set the options
    pub fn options<F>(mut self, f: F) -> Self
    where
        F: FnOnce(
            mongodb::options::InsertManyOptionsBuilder<((), (), (), ())>,
        ) -> mongodb::options::InsertManyOptions,
    {
        self.options = Some(f(mongodb::options::InsertManyOptions::builder()));
        self
    }
}

/// Operation for inserting one document
pub struct InsertOne<M: ManggaDoc> {
    doc: M::Model,
    options: Option<mongodb::options::InsertOneOptions>,
    _model: std::marker::PhantomData<M>,
}

impl<M: ManggaDoc> InsertOne<M> {
    /// Create a new `InsertOne` operation
    pub fn new(doc: M::Model) -> Self {
        Self {
            doc,
            options: None,
            _model: std::marker::PhantomData,
        }
    }

    /// Set the options
    pub fn options<F>(mut self, f: F) -> Self
    where
        F: FnOnce(
            mongodb::options::InsertOneOptionsBuilder<((), (), ())>,
        ) -> mongodb::options::InsertOneOptions,
    {
        self.options = Some(f(mongodb::options::InsertOneOptions::builder()));
        self
    }
}

impl<M: ManggaDoc> Executable for InsertMany<M> {
    type Model = M;
    type Output = ();

    async fn execute(self) -> Result<Self::Output> {
        let col = M::Model::collection()?;
        if let Some(options) = self.options {
            col.insert_many(self.docs).with_options(options).await?;
        } else {
            col.insert_many(self.docs).await?;
        }
        Ok(())
    }
}

impl<M: ManggaDoc> Executable for InsertOne<M> {
    type Model = M;
    type Output = ();

    async fn execute(self) -> Result<Self::Output> {
        let col = M::Model::collection()?;
        if let Some(options) = self.options {
            col.insert_one(self.doc).with_options(options).await?;
        } else {
            col.insert_one(self.doc).await?;
        }
        Ok(())
    }
}
