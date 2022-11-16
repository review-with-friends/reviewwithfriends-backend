use actix_web::{
    error::ErrorInternalServerError,
    get,
    http::header::ContentType,
    post,
    web::{Bytes, Data, Query},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;
use tokio::io::AsyncReadExt;

use crate::db::get_ping;

use images::{ByteStream, GetObjectError, GetObjectRequest, PutObjectRequest, S3Client, S3};

const PING_ID: &str = "123";

#[get("")]
pub async fn ping(pool: Data<MySqlPool>) -> Result<impl Responder> {
    let ping_res = get_ping(&pool, PING_ID).await;

    match ping_res {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(_) => return Err(ErrorInternalServerError("failed to ping")),
    }
}

#[derive(Deserialize)]
pub struct ImageRequest {
    image_name: String,
}

#[get("/pic")]
pub async fn pic(
    s3_client: Data<S3Client>,
    image_request: Query<ImageRequest>,
) -> Result<HttpResponse> {
    let get_output = s3_client
        .get_object(GetObjectRequest {
            bucket: "bout-dev".to_string(),
            key: image_request.image_name.clone(),
            ..Default::default()
        })
        .await
        .unwrap();

    let mut buf: Vec<u8> = Vec::new();

    let object = get_output
        .body
        .unwrap()
        .into_async_read()
        .read_to_end(&mut buf)
        .await;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::jpeg())
        .body(buf))
}

#[post("/pic")]
pub async fn upload_pic(s3_client: Data<S3Client>, bytes: Bytes) -> Result<HttpResponse> {
    s3_client
        .put_object(PutObjectRequest {
            body: Some(ByteStream::from(<Vec<u8>>::from(bytes))),
            bucket: "bout-dev".to_string(),
            key: format!("pic2.jpeg",).to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

    Ok(HttpResponse::Ok().finish())
}
