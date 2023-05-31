use crate::{
    authorization::AuthenticatedUser,
    db::{get_all_pics, get_review, remove_review_and_children, Review},
    pic_v1::shared_utils::best_effort_delete_pic,
    tracing::add_error_span,
};
use actix_web::{
    post,
    web::{Data, Query, ReqData},
    HttpResponse, Responder, Result,
};
use images::S3Client;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Deserialize, Serialize)]
pub struct RemoveReviewRequest {
    pub review_id: String,
}

/// Allows a user to remove their review.
#[post("/remove_review")]
pub async fn remove_review(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    s3_client: Data<S3Client>,
    remove_review_request: Query<RemoveReviewRequest>,
) -> Result<impl Responder> {
    let review_res = get_review(
        &pool,
        &authenticated_user.0,
        &remove_review_request.review_id,
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
        Err(error) => {
            add_error_span(&error);
            return Ok(HttpResponse::InternalServerError().body("failed to get review"));
        }
    }

    if review.user_id != authenticated_user.0 {
        return Ok(HttpResponse::Forbidden().body("you did not create this review"));
    }

    if let Err(error) = remove_review_and_children(&pool, &remove_review_request.review_id).await {
        add_error_span(&error);
        return Ok(HttpResponse::InternalServerError().body("unable to delete records"));
    }

    let pics_res = get_all_pics(&pool, &review.id).await;

    match pics_res {
        Ok(pics) => {
            for pic in pics {
                best_effort_delete_pic(&s3_client, &pool, &pic.id).await;
            }
        }
        Err(_) => {}
    }

    return Ok(HttpResponse::Ok().finish());
}
