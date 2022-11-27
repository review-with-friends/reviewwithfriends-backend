use crate::{
    authorization::AuthenticatedUser,
    db::{delete_reply, get_review},
};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Query, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct RemoveReplyRequest {
    reply_id: String,
    review_id: String,
}

#[post("/remove")]
pub async fn remove_reply(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    add_reply_request: Query<RemoveReplyRequest>,
) -> Result<impl Responder> {
    if let Err(_) = get_review(&pool, &authenticated_user.0, &add_reply_request.review_id).await {
        return Err(ErrorInternalServerError(
            "unable to find review".to_string(),
        ));
    }

    let delete_res = delete_reply(
        &pool,
        &add_reply_request.reply_id,
        &add_reply_request.review_id,
        &authenticated_user.0,
    )
    .await;

    match delete_res {
        Ok(_) => {
            return Ok(HttpResponse::Ok().finish());
        }
        Err(_) => return Err(ErrorInternalServerError("unable delete reply".to_string())),
    }
}
