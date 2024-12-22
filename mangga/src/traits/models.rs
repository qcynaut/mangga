use crate::{
    db::get_database,
    types::{BoxFut, ID},
    Result,
};
use bson::{doc, Document};
use futures::TryStreamExt;
use mongodb::{options::IndexOptions, IndexModel};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Model
///
/// Represents a struct of mangga model
pub trait Model: Clone + Send + Sync + 'static {
    /// Name of the model
    const MODEL_NAME: &'static str;

    /// Database name of the model
    const DB_NAME: &'static str;

    /// Indexes
    ///
    /// Represents the indexes of the model with this order
    /// field, name, score, unique, exp
    const INDEXES: &'static [(&'static str, &'static str, i32, bool, Option<u64>)];

    /// Get id
    fn id(&self) -> impl Into<ID>;

    /// Get mongodb collection
    #[tracing::instrument(level = tracing::Level::DEBUG)]
    fn get_collection() -> Result<mongodb::Collection<Self>> {
        Ok(get_database(Self::DB_NAME)?.collection(Self::MODEL_NAME))
    }

    /// Runs an aggregation pipeline
    #[tracing::instrument(level = tracing::Level::DEBUG)]
    fn aggregate(pipeline: Vec<Document>) -> BoxFut<Vec<Document>> {
        Box::pin(async move {
            let db = get_database(Self::DB_NAME)?;
            let col = db.collection::<Self>(Self::MODEL_NAME);
            let res = col
                .aggregate(pipeline)
                .await?
                .try_collect::<Vec<_>>()
                .await?;
            Ok(res)
        })
    }

    /// Setup the model
    #[tracing::instrument(level = tracing::Level::DEBUG)]
    fn setup() -> BoxFut<()> {
        Box::pin(async move {
            let db = get_database(Self::DB_NAME)?;
            let cols = db.list_collection_names().await?;
            if !cols.contains(&Self::MODEL_NAME.to_string()) {
                db.create_collection(Self::MODEL_NAME).await?;
            }
            let col = db.collection::<Self>(Self::MODEL_NAME);
            let mut all_indexes = col
                .list_index_names()
                .await?
                .into_iter()
                .filter(|n| n != "_id_")
                .collect::<Vec<_>>();
            all_indexes.sort();
            let mut local_indexes = Self::INDEXES
                .into_iter()
                .map(|(_, name, _, _, _)| name.to_string())
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
                        let field = field.to_string();
                        let index_options_builder = IndexOptions::builder()
                            .name(name.to_string())
                            .unique(*unique);

                        let index_options = if let Some(exp) = exp {
                            index_options_builder
                                .expire_after(Duration::from_secs(*exp))
                                .build()
                        } else {
                            index_options_builder.build()
                        };

                        indexes.push(
                            IndexModel::builder()
                                .keys(doc! {field: score})
                                .options(index_options)
                                .build(),
                        );
                    }
                }

                col.create_indexes(indexes).await?;
            }

            Ok(())
        })
    }
}

/// Dsl
///
/// Represents the dsl of the model
pub trait Dsl<T: Model> {}

/// Field
///
/// Represents a field of the model
pub trait Field {
    /// Model type of the field
    type Model: Model;

    /// Name of the field
    const NAME: &'static str;

    /// Type of the field
    type Type: Serialize + for<'de> Deserialize<'de>;
}
