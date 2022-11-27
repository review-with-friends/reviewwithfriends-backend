use crate::{
    authorization::AuthenticatedUser,
    db::{get_review, remove_like},
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
pub struct LikeReviewRequest {
    pub review_id: String,
}

#[post("/unlike")]
pub async fn unlike_review(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    like_review_request: Query<LikeReviewRequest>,
) -> Result<impl Responder> {
    if let Err(_) = get_review(&pool, &authenticated_user.0, &like_review_request.review_id).await {
        return Err(ErrorInternalServerError(
            "unable to find review".to_string(),
        ));
    }

    let remove_res =
        remove_like(&pool, &authenticated_user.0, &like_review_request.review_id).await;

    match remove_res {
        Ok(_) => {
            return Ok(HttpResponse::Ok().finish());
        }
        Err(_) => {
            return Err(ErrorInternalServerError("failed to remove like"));
        }
    }
}
