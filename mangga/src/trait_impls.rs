use crate::{
    traits::{AsExpression, Expression, Field, ManggaDoc, ManggaQuery},
    IntoAggregate,
};
use serde::Serialize;

impl<T> Expression for T
where
    T: Field + std::clone::Clone,
{
    fn build(self) -> bson::Bson {
        bson::Bson::String(Self::NAME.to_string())
    }
}

impl Expression for bson::Bson {
    fn build(self) -> bson::Bson {
        self
    }
}

impl<T> AsExpression for T
where
    T: Serialize,
{
    fn as_expression(self) -> impl Expression {
        bson::to_bson(&self).unwrap()
    }
}

impl<T: ManggaDoc> ManggaQuery for T {}

impl IntoAggregate for mongodb::options::FindOneOptions {
    fn pipeline(self) -> Vec<bson::Document> {
        let mut pipeline = vec![];
        if let Some(skip) = self.skip {
            let skip = skip as i64;
            pipeline.push(bson::doc! { "$skip": skip });
        }
        if let Some(sort) = self.sort {
            pipeline.push(bson::doc! { "$sort": sort });
        }
        pipeline
    }
}

impl IntoAggregate for mongodb::options::FindOptions {
    fn pipeline(self) -> Vec<bson::Document> {
        let mut pipeline = vec![];
        if let Some(skip) = self.skip {
            let skip = skip as i64;
            pipeline.push(bson::doc! { "$skip": skip });
        }
        if let Some(sort) = self.sort {
            pipeline.push(bson::doc! { "$sort": sort });
        }
        if let Some(limit) = self.limit {
            pipeline.push(bson::doc! { "$limit": limit });
        }
        pipeline
    }
}
