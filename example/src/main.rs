use chrono::Duration;
use mangga::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[mangga(name = "users", db = "db1")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ID,
    #[index(unique = true)]
    pub email: String,
    pub name: String,
    #[index(exp = 0)]
    pub expired: DateTime,
}

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[mangga(name = "users", db = "db2")]
pub struct User2 {
    #[serde(rename = "_id")]
    pub id: ID,
    #[index(unique = true)]
    pub email: String,
    pub name: String,
    #[index(exp = 0)]
    pub expired: DateTime,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    connect_database("mongodb://localhost:27017", vec!["db1", "db2"]).await?;
    User::setup().await?;
    let exp = DateTime::now() + Duration::minutes(2);

    let users = vec![
        User::new(ID::default(), "u1@u1.com", "John Doe", exp),
        User::new(ID::default(), "u2@u2.com", "Jane Doe", exp),
        User::new(ID::default(), "u3@u3.com", "John Smith", exp),
    ];

    let users2 = vec![
        User2::new(ID::default(), "u4@u4.com", "John Doe", exp),
        User2::new(ID::default(), "u5@u5.com", "Jane Doe", exp),
        User2::new(ID::default(), "u6@u6.com", "John Smith", exp),
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
    let users = User::dsl
        .find_many(())
        .opts(|o| o.sort(User::name.asc()).build())
        .await?;
    let users2 = User2::dsl.find_many(()).await?;
    assert_eq!(users.len(), 3);
    assert_eq!(users2.len(), 3);

    println!("users: {:#?}", users);
    println!("users2: {:#?}", users2);

    // update all users
    for user in users {
        User::dsl
            .update_one(User::id.eq(user.id), vec![User::name.set("updated")])
            .await?;
    }
    User2::dsl
        .update_many(
            User2::id.is_in(users2.iter().map(|user| user.id)),
            vec![User::name.set("updated")],
        )
        .await?;

    let users = User::dsl.find_many(()).await?;
    let users2 = User2::dsl.find_many(()).await?;
    assert_eq!(users.len(), 3);
    assert_eq!(users2.len(), 3);
    assert_eq!(users.iter().all(|user| user.name == "updated"), true);
    assert_eq!(users2.iter().all(|user| user.name == "updated"), true);

    // should conflict
    let res = User::dsl.update_one(User::id.eq(users[1].id), vec![User::email.set("u1@u1.com")]).await;
    if let Err(err) = res {
        println!("conflict: {}", err.is_conflict());
        return Err(err.into());
    }

    Ok(())
}
