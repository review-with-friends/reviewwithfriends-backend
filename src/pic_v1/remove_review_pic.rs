use super::shared_utils::best_effort_delete_pic;
use crate::{
    authorization::AuthenticatedUser,
    db::{get_review, remove_review_pic_id, Review},
};
use actix_web::{
    post,
    web::{Data, Query, ReqData},
    HttpResponse, Result,
};
use images::S3Client;
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct RemoveReviewPicRequest {
    review_id: String,
}

#[post("/remove_review_pic")]
pub async fn remove_review_pic(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    s3_client: Data<S3Client>,
    remove_review_pic_request: Query<RemoveReviewPicRequest>,
) -> Result<HttpResponse> {
    let review_res = get_review(
        &pool,
        &authenticated_user.0,
        &remove_review_pic_request.review_id,
    )
    .await;

    let review: Review;
    match review_res {
        Ok(review_opt) => {
            if let Some(review_tmp) = review_opt {
                review = review_tmp;
            } else {
                return Ok(HttpResponse::NotFound().body("could not find review"));
            }
        }
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().body("failed to get review"));
        }
    }

    if review.user_id != authenticated_user.0 {
        return Ok(HttpResponse::BadRequest().body("unable to edit this review"));
    }

    if let None = review.pic_id {
        return Ok(HttpResponse::Ok().finish());
    }

    if let Err(_) = remove_review_pic_id(&pool, &review.id).await {
        return Ok(HttpResponse::InternalServerError().body("unable to remove pic"));
    }

    if let Some(pic_id) = review.pic_id {
        best_effort_delete_pic(&s3_client, &pool, &pic_id).await;
    }

    return Ok(HttpResponse::Ok().finish());
}
