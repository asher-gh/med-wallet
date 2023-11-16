use crate::*;

// TODO: add the document with authenticated user's id

#[derive(Debug)]
struct Doc {
    name: Option<String>,
    content: Option<Vec<u8>>,
}

// upload with name
pub async fn upload_file(
    State(pool): State<PgPool>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        tracing::debug!("Length of `{}` is {} bytes", name, data.len());

        query!(
            "INSERT INTO document (owner_id, name, content) VALUES (1, $1, $2)",
            name,
            &data[..]
        )
        .execute(&pool)
        .await
        .unwrap();
    }

    Ok(StatusCode::OK)
}

// TODO: Authenticate request
// TODO: return content and filename
pub async fn serve_asset(
    Path(doc_id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, StatusCode> {
    // Query DB to get the asset
    let doc = query_as!(
        Doc,
        "SELECT name, content from document where id = $1",
        doc_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Response::builder()
        .status(StatusCode::OK)
        // .header("Content-Type", "application/octet-stream")
        .header(
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", doc.name.unwrap_or_default()),
        )
        .body(Body::from(doc.content.unwrap_or_default()))
        .map_err(|_| StatusCode::NOT_FOUND)
}
