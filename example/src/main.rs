use mangga::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Model, Default, Serialize, Deserialize)]
#[mangga(name = "users", db = "db1")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ID,
    pub name: String,
}

#[derive(Debug, Clone, Model, Default, Serialize, Deserialize)]
#[mangga(name = "users", db = "db2")]
pub struct User2 {
    #[serde(rename = "_id")]
    pub id: ID,
    pub name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    connect_database("mongodb://localhost:27017", vec!["db1", "db2"]).await?;
    
    let users = vec![
        User::new(ID::default(), "John Doe"),
        User::new(ID::default(), "Jane Doe"),
        User::new(ID::default(), "Joe Doe"),
    ];

    let users2 = vec![
        User2::new(ID::default(), "John Doe"),
        User2::new(ID::default(), "Jane Doe"),
        User2::new(ID::default(), "Joe Doe"),
    ];

    // insert all users
    User::dsl.insert_many(users).await?;
    User2::dsl.insert_many(users2).await?;

    // verify if all users are inserted
    let count = User::dsl.count(()).await?;
    let count2 = User2::dsl.count(()).await?;
    assert_eq!(count, 3);
    assert_eq!(count2, 3);

    // find all users
    let users = User::dsl.find_many(()).await?;
    let users2 = User2::dsl.find_many(()).await?;
    assert_eq!(users.len(), 3);
    assert_eq!(users2.len(), 3);

    println!("users: {:?}", users);
    println!("users2: {:?}", users2);

    // delete all users
    User::dsl.delete_many(()).await?;
    User2::dsl.delete_many(()).await?;

    Ok(())
}
