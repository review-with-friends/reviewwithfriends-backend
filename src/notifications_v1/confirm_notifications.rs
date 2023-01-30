use crate::{authorization::AuthenticatedUser, db};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, ReqData},
    HttpResponse, Responder, Result,
};
use sqlx::MySqlPool;

/// Acknowledges all notifications have been received by the client.
#[post("")]
pub async fn confirm_notifications(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
) -> Result<impl Responder> {
    let reply_res = db::confirm_notifications(&pool, &authenticated_user.0).await;

    match reply_res {
        Ok(_) => {
            return Ok(HttpResponse::Ok().finish());
        }
        Err(_) => {
            return Err(ErrorInternalServerError(
                "unable confirm notifications".to_string(),
            ))
        }
    }
}
