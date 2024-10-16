use crate::{Error, Result};
use once_cell::sync::OnceCell;
use std::collections::HashMap;

/// Client
///
/// Represents a MongoDB client
struct Client {
    #[allow(dead_code)]
    c: mongodb::Client,
    db: HashMap<String, mongodb::Database>,
}

/// MANGGA stores the database connection
static MANGGA: OnceCell<Client> = OnceCell::new();

/// Initialize the database connection
///
/// This function must be called before using the database
///
/// # Arguments
///
/// * `uri` - The URI of the database
/// * `databases` - List of database names
///
/// # Examples
///
/// ```no_run
/// use mangga::prelude::*;
///
/// let db_uri = "mongodb://localhost:27017";
/// let databases = vec!["db1", "db2"];
/// connect_database(db_uri, databases).await;
/// ```
pub async fn connect_database<U: AsRef<str>, D: IntoIterator<Item = S>, S: AsRef<str>>(
    uri: U,
    databases: D,
) -> Result<()> {
    let client = mongodb::Client::with_uri_str(uri.as_ref()).await?;
    let mut db = HashMap::new();
    for db_name in databases.into_iter() {
        db.insert(
            db_name.as_ref().to_string(),
            client.database(db_name.as_ref()),
        );
    }

    MANGGA
        .set(Client { c: client, db })
        .map_err(|_| Error::Init("Failed to set MANGGA".to_string()))?;

    Ok(())
}

/// Get the database
pub fn get_database<T: AsRef<str>>(name: T) -> Result<mongodb::Database> {
    MANGGA
        .get()
        .ok_or_else(|| Error::Init("Failed to get MANGGA".to_string()))?
        .db
        .get(name.as_ref())
        .ok_or_else(|| Error::Init("Failed to get database".to_string()))
        .cloned()
}
