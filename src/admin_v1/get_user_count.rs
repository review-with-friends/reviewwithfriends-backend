use crate::{authorization::AuthenticatedUser, db::get_total_user_count};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Json, ReqData},
    Responder, Result,
};
use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Serialize)]
struct Count {
    count: i64,
}

/// Gets the total users registered for the app.
#[get("total_user_count")]
pub async fn get_user_count(
    _authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
) -> Result<impl Responder> {
    let reply_res = get_total_user_count(&pool).await;

    match reply_res {
        Ok(count) => {
            return Ok(Json(Count { count }));
        }
        Err(_) => return Err(ErrorInternalServerError("unable to get likes".to_string())),
    }
}
