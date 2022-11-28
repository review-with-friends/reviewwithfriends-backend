use crate::db::get_ping;
use actix_web::{error::ErrorInternalServerError, get, web::Data, HttpResponse, Responder, Result};
use sqlx::MySqlPool;

const PING_ID: &str = "123";

/// Simple API for validating db connectivity.
#[get("")]
pub async fn ping(pool: Data<MySqlPool>) -> Result<impl Responder> {
    let ping_res = get_ping(&pool, PING_ID).await;

    match ping_res {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(_) => return Err(ErrorInternalServerError("failed to ping")),
    }
}
