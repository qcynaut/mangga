use crate::{Executable, ManggaDoc, Model};

/// Query for counting documents
pub struct Count<M: ManggaDoc> {
    filter: bson::Bson,
    options: Option<mongodb::options::CountOptions>,
    _model: std::marker::PhantomData<M>,
}

impl<M: ManggaDoc> Count<M> {
    /// Create a new `Count` query
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
            mongodb::options::CountOptionsBuilder<((), (), (), (), (), (), (), ())>,
        ) -> mongodb::options::CountOptions,
    {
        self.options = Some(f(mongodb::options::CountOptions::builder()));
        self
    }
}

impl<M: ManggaDoc> Executable for Count<M> {
    type Model = M;
    type Output = u64;

    async fn execute(self) -> crate::error::Result<Self::Output> {
        let col = M::Model::collection()?;
        let filter = self.filter.as_document().cloned().unwrap_or_default();
        let count = if let Some(options) = self.options {
            col.count_documents(filter).with_options(options)
        } else {
            col.count_documents(filter)
        };

        Ok(count.await?)
    }
}
