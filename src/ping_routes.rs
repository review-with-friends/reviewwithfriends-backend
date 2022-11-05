use actix_web::{error::ErrorInternalServerError, get, web::Data, HttpResponse, Responder, Result};
use sqlx::MySqlPool;

use crate::db::get_ping;

const PING_ID: &str = "123";

#[get("")]
pub async fn ping(pool: Data<MySqlPool>) -> Result<impl Responder> {
    let ping_res = get_ping(&pool, PING_ID).await;

    match ping_res {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(_) => return Err(ErrorInternalServerError("failed to ping")),
    }
}
