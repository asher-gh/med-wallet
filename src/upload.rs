use axum::{
    extract::{Multipart, State},
    response::{Html, IntoResponse},
};
use sqlx::{query, query_as, PgPool};

// TODO: add the document with authenticated user's id

pub async fn upload(State(pool): State<PgPool>, mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());

        query!(
            "INSERT INTO document (owner_id, content) VALUES (1, $1)",
            &data[..]
        )
        .execute(&pool)
        .await
        .unwrap();
    }
}

#[derive(Debug)]
struct Content {
    content: Option<Vec<u8>>,
}

// TODO: respond client with the document
pub async fn download(State(pool): State<PgPool>) {
    let content = query_as!(Content, "SELECT content from document where id = $1", 1)
        .fetch_one(&pool)
        .await
        .unwrap();

    // testing data integrity
    // if let Some(data) = content.content {
    //     std::fs::write("foo.bin", data).unwrap();
    // }
}
