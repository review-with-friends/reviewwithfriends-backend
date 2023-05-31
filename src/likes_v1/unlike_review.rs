use crate::{
    authorization::AuthenticatedUser,
    db::{get_review, remove_like},
    tracing::add_error_span,
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

/// Allows users to unlike a review.
#[post("/unlike")]
pub async fn unlike_review(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    like_review_request: Query<LikeReviewRequest>,
) -> Result<impl Responder> {
    let review_res = get_review(&pool, &authenticated_user.0, &like_review_request.review_id).await;

    match review_res {
        Ok(review_opt) => {
            if let None = review_opt {
                return Err(ErrorNotFound("could not find review"));
            }
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError("failed to get review"));
        }
    }

    let remove_res =
        remove_like(&pool, &authenticated_user.0, &like_review_request.review_id).await;

    match remove_res {
        Ok(_) => {
            return Ok(HttpResponse::Ok().finish());
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError("failed to remove like"));
        }
    }
}
