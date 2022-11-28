use crate::{
    authorization::AuthenticatedUser,
    db::{create_like, get_review, is_already_liked},
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

    match review_res {
        Ok(review_opt) => {
            if let None = review_opt {
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
            return Ok(HttpResponse::Ok().finish());
        }
        Err(_) => {
            return Err(ErrorInternalServerError("failed to create review"));
        }
    }
}
