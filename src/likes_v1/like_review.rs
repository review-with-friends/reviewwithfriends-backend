use crate::{
    authorization::AuthenticatedUser,
    db::{create_like, create_notification, get_review, is_already_liked, Review},
    notifications_v1::ActionType,
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    post,
    web::{Data, Query, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct LikeReviewRequest {
    pub review_id: String,
}

/// Allows users to like a review.
#[post("")]
pub async fn like_review(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    like_review_request: Query<LikeReviewRequest>,
) -> Result<impl Responder> {
    let review_res = get_review(&pool, &authenticated_user.0, &like_review_request.review_id).await;

    let review: Review;
    match review_res {
        Ok(review_opt) => {
            if let Some(review_tmp) = review_opt {
                review = review_tmp
            } else {
                return Err(ErrorNotFound("could not find review"));
            }
        }
        Err(_) => return Err(ErrorInternalServerError("failed to get review")),
    }

    let already_created_res =
        is_already_liked(&pool, &authenticated_user.0, &like_review_request.review_id).await;

    match already_created_res {
        Ok(is_liked) => {
            if is_liked {
                return Ok(HttpResponse::Ok().finish());
            }
        }
        Err(_) => {
            return Err(ErrorInternalServerError("failed to fetch existing likes"));
        }
    }

    let create_res =
        create_like(&pool, &authenticated_user.0, &like_review_request.review_id).await;

    match create_res {
        Ok(_) => {
            // Creating the notificaiton is best effort. We may look into not awaiting this;
            // though unsure of how the tokio runtime closes out the webrequest.
            let _ = create_notification(
                &pool,
                &authenticated_user.0,
                &review.user_id,
                &review.id,
                ActionType::Like.into(),
            )
            .await;

            return Ok(HttpResponse::Ok().finish());
        }
        Err(_) => {
            return Err(ErrorInternalServerError("failed to create review"));
        }
    }
}
