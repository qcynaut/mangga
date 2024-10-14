use crate::error::{Error, Result};
use once_cell::sync::OnceCell;

/// Stores
///
/// Contains the database connection and the database
/// inside once cell.
struct Client {
    db: mongodb::Database,
}

/// MANGGA stores the database connection and the database
/// inside once cell.
static MANGGA: OnceCell<Client> = OnceCell::new();

/// Initialize the database connection
///
/// This function should be called before using the database
pub async fn connect_database<T: AsRef<str>>(uri: T, database: T) -> Result<()> {
    let client = mongodb::Client::with_uri_str(uri.as_ref()).await?;
    let db = client.database(database.as_ref());

    MANGGA
        .set(Client { db })
        .map_err(|_| Error::InitializationError)?;

    Ok(())
}

/// Get the database connection
pub fn get_database() -> Result<mongodb::Database> {
    MANGGA
        .get()
        .ok_or(Error::UninitializedDatabase)
        .map(|client| client.db.clone())
}
