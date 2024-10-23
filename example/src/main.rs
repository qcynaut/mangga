use bson::doc;
use mangga::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[mangga(name = "users", db = "db1")]
#[graphql()]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ID,
    #[index(unique = true)]
    pub email: String,
    pub name: String,
}

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[mangga(name = "books", db = "db1")]
pub struct Book {
    #[serde(rename = "_id")]
    pub id: ID,
    pub title: String,
    pub author: String,
    #[index(score = 1)]
    #[graphql(rel = {name: "user", model: User})]
    pub user_id: ID,
    #[index(score = 1)]
    #[graphql(rel = {name: "store", model: Store})]
    pub store_id: ID,
}

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[mangga(name = "stores", db = "db1")]
#[graphql()]
pub struct Store {
    #[serde(rename = "_id")]
    pub id: ID,
    pub name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    connect_database("mongodb://localhost:27017", vec!["db1"]).await?;
    User::setup().await?;
    Book::setup().await?;
    Store::setup().await?;

    clean().await?;

    let default_users = vec![
        User::new(ID::default(), "john@example.com", "John Doe"),
        User::new(ID::default(), "jane@example.com", "Jane Doe"),
    ];

    User::dsl.insert_many(default_users.clone()).await?;

    let first_store = Store::new(ID::default(), "Store 1");
    first_store.insert().await?;
    let second_store = Store::new(ID::default(), "Store 2");
    second_store.insert().await?;

    let mut books = vec![];

    for user in default_users {
        books.push(Book::new(
            ID::default(),
            "The Great Gatsby",
            "F. Scott Fitzgerald",
            user.id,
            second_store.id,
        ));
        books.push(Book::new(
            ID::default(),
            "To Kill a Mockingbird",
            "Harper Lee",
            user.id,
            first_store.id,
        ));
        books.push(Book::new(
            ID::default(),
            "1984",
            "George Orwell",
            user.id,
            first_store.id,
        ));
    }

    Book::dsl.insert_many(books).await?;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct ResBook {
        _id: ID,
        title: String,
        author: String,
        user_id: ID,
        store_id: ID,
        store: Store,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Res {
        _id: ID,
        name: String,
        books: Vec<ResBook>,
    }

    let res = User::aggregate(vec![
        doc! {
            "$match": {
                "email": "john@example.com"
            }
        },
        doc! {
            "$lookup": {
                "from": "books",
                "localField": "_id",
                "foreignField": "user_id",
                "as": "books"
            }
        },
        doc! {
            "$lookup": {
                "from": "stores",
                "let": { "books": "$books" },
                "pipeline": [
                    {
                        "$match": {
                            "$expr": {
                                "$in": ["$_id", {
                                    "$map": {
                                        "input": "$$books",
                                        "as": "book",
                                        "in": "$$book.store_id"
                                    }
                                }]
                            }
                        }
                    }
                ],
                "as": "stores"
            }
        },
        doc! {
            "$project": {
                "_id": 1,
                "name": 1,
                "books": {
                    "$map": {
                        "input": "$books",
                        "as": "book",
                        "in": {
                            "_id": "$$book._id",
                            "title": "$$book.title",
                            "author": "$$book.author",
                            "user_id": "$$book.user_id",
                            "store_id": "$$book.store_id",
                            // filter store by id
                            "store": {
                                "$filter": {
                                    "input": "$stores",
                                    "as": "store",
                                    "cond": {
                                        "$eq": ["$$store._id", "$$book.store_id"]
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    ])
    .await?;

    println!("{:#?}", res);
    clean().await?;

    Ok(())
}

async fn clean() -> Result<(), Box<dyn std::error::Error>> {
    User::dsl.delete_many(()).await?;
    Book::dsl.delete_many(()).await?;
    Store::dsl.delete_many(()).await?;
    Ok(())
}
