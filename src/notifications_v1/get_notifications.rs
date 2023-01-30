use super::notification_types::NotificationPub;
use crate::{authorization::AuthenticatedUser, db};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Json, ReqData},
    Responder, Result,
};
use sqlx::MySqlPool;

/// Gets the top 50 latest notifications for a user.
#[get("")]
pub async fn get_notifications(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
) -> Result<impl Responder> {
    let notifications_res = db::get_notifications(&pool, &authenticated_user.0).await;

    match notifications_res {
        Ok(notifications) => {
            let notifications_pub: Vec<NotificationPub> = notifications
                .into_iter()
                .map(|f| -> NotificationPub { f.into() })
                .collect();
            return Ok(Json(notifications_pub));
        }
        Err(_) => {
            return Err(ErrorInternalServerError(
                "unable to get notifications".to_string(),
            ))
        }
    }
}
