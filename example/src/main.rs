use mangga::prelude::*;
use serde::{Deserialize, Serialize};

#[model("users", graphql, refs = {books: {target: book::doc, array: true, target_field: "user_id"}})]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    #[model(index = {unique: true})]
    pub email: String,
    pub age: i32,
    pub created_at: DateTime,
}

#[model("categories", graphql)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub description: String,
}

#[model("books", graphql)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub description: String,
    #[model(ref = {name: "user",target: user::doc, array: false})]
    pub user_id: ID,
    #[model(ref = {name: "category",target: category::doc, array: false})]
    pub category_id: ID,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database = std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    connect_database(db_uri, database).await.unwrap();

    user::doc.init().await.unwrap();

    let users = vec![
        User::new("John Doe", "j@j.com", 30, DateTime::now()),
        User::new("Jane Doe", "ja@j.com", 29, DateTime::now()),
    ];
    let categories = vec![
        Category::new("Category 1", "Description 1"),
        Category::new("Category 2", "Description 2"),
    ];
    let mut books = vec![];
    for user in &users {
        books.push(Book::new(
            "Book 1",
            "Description 1",
            user.id.clone(),
            categories[0].id.clone(),
        ));
        books.push(Book::new(
            "Book 2",
            "Description 2",
            user.id.clone(),
            categories[1].id.clone(),
        ));
    }
    user::doc.insert(users).execute().await.unwrap();
    category::doc.insert(categories).execute().await.unwrap();
    book::doc.insert(books).execute().await.unwrap();

    let books = book::doc
        .filter(book::title.eq("Book 1"))
        .find()
        .options(|opts| opts.sort(book::title.asc()).build())
        .execute()
        .await
        .unwrap();
    for book in books {
        println!("{}", book.title);
    }

    book::doc::raw_filter("title", "$eq", "Book 1")
        .update_many(BookUpdate::new().title("Book 0"))
        .execute()
        .await
        .unwrap();

    let books = book::doc
        .find()
        .options(|opts| opts.sort(book::title.asc()).build())
        .execute()
        .await
        .unwrap();
    for book in books {
        println!("{}", book.title);
    }

    book::doc
        .update(BookUpdate::new().title("MyBook"))
        .execute()
        .await
        .unwrap();

    let books = book::doc
        .find()
        .options(|opts| opts.sort(book::title.asc()).build())
        .execute()
        .await
        .unwrap();
    for book in books {
        println!("{:#?}", book);
    }

    book::doc.delete().execute().await.unwrap();
    user::doc.delete().execute().await.unwrap();
    category::doc.delete().execute().await.unwrap();
}
