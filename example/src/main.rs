use mangga::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Model, Default, Serialize, Deserialize)]
#[mangga(name = "users", db = "db1")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ID,
    pub name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    connect_database("mongodb://localhost:27017", vec!["db1", "db2"]).await?;
    let user = User::new(ID::default(), "John Doe");
    User::dsl()
        .insert_one(&user)
        .await?;
    user.delete().await?;
    println!("{:?}", user);
    Ok(())
}
