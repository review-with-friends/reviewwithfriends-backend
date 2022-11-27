use crate::{
    authorization::AuthenticatedUser,
    db::{get_all_replies, get_review},
};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Json, Query, ReqData},
    Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

use super::reply_types::ReplyPub;

#[derive(Deserialize)]
pub struct GetRepliesRequest {
    review_id: String,
}

#[get("")]
pub async fn get_replies(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    get_replies_request: Query<GetRepliesRequest>,
) -> Result<impl Responder> {
    if let Err(_) = get_review(&pool, &authenticated_user.0, &get_replies_request.review_id).await {
        return Err(ErrorInternalServerError(
            "unable to find review".to_string(),
        ));
    }

    let reply_res = get_all_replies(&pool, &get_replies_request.review_id).await;

    match reply_res {
        Ok(likes) => {
            let likes_pub: Vec<ReplyPub> = likes
                .into_iter()
                .map(|f| -> ReplyPub { f.into() })
                .collect();
            return Ok(Json(likes_pub));
        }
        Err(_) => return Err(ErrorInternalServerError("unable to get likes".to_string())),
    }
}
