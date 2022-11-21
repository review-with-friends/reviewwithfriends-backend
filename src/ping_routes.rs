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

use images::{ByteStream, GetObjectRequest, PutObjectRequest, S3Client, S3};

const PING_ID: &str = "123";

#[get("")]
pub async fn ping(pool: Data<MySqlPool>) -> Result<impl Responder> {
    let ping_res = get_ping(&pool, PING_ID).await;

    match ping_res {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(_) => return Err(ErrorInternalServerError("failed to ping")),
    }
}
