use crate::{client::get_database, dsl, error::Result, types::ID};
use serde::{Deserialize, Serialize};

/// Identifiable
pub trait Identifiable {
    /// Returns the identifier
    fn mid(&self) -> &ID;
}

/// Model
pub trait Model:
    Identifiable + Default + Serialize + for<'de> Deserialize<'de> + Send + Sync
{
    const COLLECTION: &'static str;

    /// Get the collection
    fn collection() -> Result<mongodb::Collection<Self>> {
        let db = get_database()?;
        Ok(db.collection(Self::COLLECTION))
    }
}

/// Field
pub trait Field: Sized {
    const NAME: &'static str;
    type Type: Serialize + for<'de> Deserialize<'de>;
    type Doc: Model;
}

/// Expression
pub trait Expression: Clone {
    fn build(self) -> bson::Bson;
}

/// AsExpression
pub trait AsExpression {
    fn as_expression(self) -> impl Expression;
}

#[allow(async_fn_in_trait)]
/// ManggaDoc
pub trait ManggaDoc: Sized {
    type Model: Model;
    const INDEXES: &'static [(&'static str, &'static str, i32, bool, Option<u64>)] = &[];

    /// Create raw filter
    fn raw_filter(
        name: &str,
        op: &str,
        value: impl Into<bson::Bson>,
    ) -> dsl::Filter<dsl::RawFilter, Self> {
        let comparisons = ["$eq", "$ne", "$gt", "$gte", "$lt", "$lte"];
        let doc = if comparisons.contains(&op) {
            bson::doc! {name: {op: value}}
        } else {
            bson::doc! {op: {name: value}}
        };
        dsl::Filter::new(dsl::RawFilter::new(doc))
    }

    /// Initialize the document
    async fn init(self) -> Result<()> {
        let database = get_database()?;
        let all_collections = database.list_collection_names().await?;
        if !all_collections.contains(&Self::Model::COLLECTION.to_string()) {
            database.create_collection(Self::Model::COLLECTION).await?;
        }
        let col = Self::Model::collection()?;
        let mut all_indexes = col
            .list_index_names()
            .await?
            .into_iter()
            .filter(|n| n != "_id_")
            .collect::<Vec<_>>();
        all_indexes.sort();
        let mut local_indexes = Self::INDEXES
            .into_iter()
            .map(|(_, name, _, _,_)| name.to_string())
            .collect::<Vec<_>>();
        local_indexes.sort();
        if all_indexes == local_indexes {
            return Ok(());
        }

        let match_indexes = all_indexes
            .iter()
            .filter(|n| local_indexes.contains(n))
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let unmatch_indexes = all_indexes
            .iter()
            .filter(|n| !local_indexes.contains(n))
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let new_indexes = local_indexes
            .iter()
            .filter(|n| !match_indexes.contains(n))
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        if !unmatch_indexes.is_empty() {
            for name in unmatch_indexes {
                col.drop_index(name).await?;
            }
        }

        if !new_indexes.is_empty() {
            let mut indexes = vec![];
            for name in new_indexes {
                let index = Self::INDEXES.iter().find(|(_, n, _, _, _)| *n == &name);
                if let Some((field, name, score, unique, exp)) = index {
                    let field = &field.to_string();
                    let score = if *score == -1 { -1 } else { 1 };
                    let index_options_builder = mongodb::options::IndexOptions::builder()
                    .name(name.to_string())
                    .unique(*unique);

                    let index_options = if let Some(exp) = exp {
                        index_options_builder.expire_after(std::time::Duration::from_secs(*exp)).build()
                    } else {
                        index_options_builder.build()
                    };
                    indexes.push(
                        mongodb::IndexModel::builder()
                            .keys(bson::doc! {field: score})
                            .options(index_options)
                            .build(),
                    );
                }
            }

            col.create_indexes(indexes).await?;
        }

        Ok(())
    }
}

/// ManggaQuery
pub trait ManggaQuery: Sized {
    fn filter<Pred>(self, pred: Pred) -> dsl::Filter<Pred, Self> {
        dsl::Filter::new(pred)
    }
}

#[allow(async_fn_in_trait)]
/// Executable
pub trait Executable: Sized {
    type Model: ManggaDoc;
    type Output;

    /// Execute the operation
    async fn execute(self) -> Result<Self::Output>;
}

#[allow(async_fn_in_trait)]
/// JoinExecutable
pub trait JoinExecutable: Sized {
    type Output;

    /// Execute the operation
    async fn execute(self) -> Result<Self::Output>;
}

#[allow(async_fn_in_trait)]
/// JoinExecutableSingle
pub trait JoinExecutableSingle: Sized {
    type Output;

    /// Execute the operation
    async fn execute(self) -> Result<Self::Output>;
}

/// IntoQuery
pub trait IntoQuery {
    type Options: IntoAggregate;

    /// Returns the query filter
    fn filter(&self) -> bson::Document;

    /// Returns the query options
    fn options(&self) -> Option<Self::Options>;
}

/// IntoAggregate
pub trait IntoAggregate {
    /// Returns the aggregation pipeline
    fn pipeline(self) -> Vec<bson::Document>;
}

/// AsUpdate
pub trait AsUpdate: Serialize + for<'de> Deserialize<'de> + Sized {
    const FIELDS: &'static [&'static str];

    /// Returns value of a field
    fn field(&self, name: &str) -> Option<bson::Bson>;

    /// Returns the update document
    fn as_update(self) -> Result<bson::Document> {
        let mut doc = bson::Document::new();

        for field in Self::FIELDS {
            if let Some(val) = self.field(field) {
                doc.insert(field.to_string(), val);
            }
        }

        Ok(bson::doc! {
            "$set": doc
        })
    }
}
