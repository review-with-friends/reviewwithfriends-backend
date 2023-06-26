use crate::{
    authorization::AuthenticatedUser,
    db::{get_all_replies, get_review},
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
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

/// Gets all the replies for a given review.
#[get("")]
pub async fn get_replies(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    get_replies_request: Query<GetRepliesRequest>,
) -> Result<impl Responder> {
    let review_res = get_review(&pool, &authenticated_user.0, &get_replies_request.review_id).await;

    match review_res {
        Ok(review_opt) => {
            if let None = review_opt {
                return Err(ErrorNotFound("could not find review".to_string()));
            }
        }
        Err(_) => return Err(ErrorInternalServerError("failed to get review".to_string())),
    }

    let reply_res = get_all_replies(&pool, &get_replies_request.review_id).await;

    match reply_res {
        Ok(replies) => {
            let reply_pub: Vec<ReplyPub> = replies
                .into_iter()
                .map(|f| -> ReplyPub { f.into() })
                .collect();
            return Ok(Json(reply_pub));
        }
        Err(_) => return Err(ErrorInternalServerError("unable to get likes".to_string())),
    }
}
