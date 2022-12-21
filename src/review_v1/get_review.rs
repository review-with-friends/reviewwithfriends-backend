use crate::{
    authorization::AuthenticatedUser,
    compound_types::CompoundReviewPub,
    db::{get_all_likes, get_all_replies, get_review},
    likes_v1::LikePub,
    reply_v1::ReplyPub,
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get,
    web::{Data, Json, Query, ReqData},
    Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

use super::review_types::ReviewPub;

#[derive(Deserialize)]
pub struct ReviewRequest {
    review_id: String,
}

/// Gets a review by the given id.
/// The returned object contains all
/// the initial information for the review.
#[get("/review_by_id")]
pub async fn get_review_by_id(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    review_request: Query<ReviewRequest>,
) -> Result<impl Responder> {
    let review: ReviewPub;
    let review_res = get_review(&pool, &authenticated_user.0, &review_request.review_id).await;

    match review_res {
        Ok(review_opt) => {
            if let Some(review_tmp) = review_opt {
                review = review_tmp.into();
            } else {
                return Err(ErrorNotFound("unable to find review"));
            }
        }
        Err(_) => return Err(ErrorInternalServerError("unable to get review")),
    }

    let likes: Vec<LikePub>;

    let likes_res = get_all_likes(&pool, &review_request.review_id).await;

    match likes_res {
        Ok(likes_tmp) => {
            likes = likes_tmp
                .into_iter()
                .map(|f| -> LikePub { f.into() })
                .collect();
        }
        Err(_) => return Err(ErrorInternalServerError("unable to get likes")),
    }

    let replies: Vec<ReplyPub>;

    let replies_res = get_all_replies(&pool, &review_request.review_id).await;

    match replies_res {
        Ok(replies_tmp) => {
            replies = replies_tmp
                .into_iter()
                .map(|f| -> ReplyPub { f.into() })
                .collect();
        }
        Err(_) => return Err(ErrorInternalServerError("unable to get replies")),
    }

    return Ok(Json(CompoundReviewPub {
        review,
        likes,
        replies,
    }));
}
