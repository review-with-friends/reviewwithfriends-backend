use crate::{
    authorization::AuthenticatedUser,
    db::{create_pic, get_review, update_review_pic_id, Review},
};
use actix_web::{
    post,
    web::{Bytes, Data, Query, ReqData},
    HttpResponse, Result,
};
use images::{ByteStream, PutObjectRequest, S3Client, S3};
use serde::Deserialize;
use sqlx::MySqlPool;
use validation::validate_review_pic;

use super::shared_utils::best_effort_delete_pic;

#[derive(Deserialize)]
pub struct AddReviewPicRequest {
    review_id: String,
}

/// Allows users to add a pic to their review.
#[post("/review_pic")]
pub async fn add_review_pic(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    s3_client: Data<S3Client>,
    add_review_pic_request: Query<AddReviewPicRequest>,
    pic_bytes: Bytes,
) -> Result<HttpResponse> {
    if let Err(err) = validate_review_pic(&pic_bytes) {
        return Ok(HttpResponse::BadRequest().body(err));
    }

    let previous_pic_id: Option<String>;
    let review_res = get_review(
        &pool,
        &authenticated_user.0,
        &add_review_pic_request.review_id,
    )
    .await;

    let review: Review;
    match review_res {
        Ok(review_opt) => {
            if let Some(review_tmp) = review_opt {
                previous_pic_id = review_tmp.pic_id.clone();
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
        return Ok(HttpResponse::InternalServerError().body("unable to edit this review"));
    }

    let pic_res = create_pic(&pool).await;

    match pic_res {
        Ok(pic) => {
            if let Err(_) = s3_client
                .put_object(PutObjectRequest {
                    body: Some(ByteStream::from(<Vec<u8>>::from(pic_bytes))),
                    bucket: "bout".to_string(),
                    key: pic.id.clone(),
                    acl: Some("public-read".to_string()),
                    ..Default::default()
                })
                .await
            {
                return Ok(HttpResponse::InternalServerError().body("unable to store review pic"));
            }

            if let Err(_) = update_review_pic_id(&pool, &pic.id, &review.id).await {
                return Ok(HttpResponse::InternalServerError().body("unable to save review pic"));
            }

            if let Some(pic_id) = previous_pic_id {
                best_effort_delete_pic(&s3_client, &pool, &pic_id).await;
                // best effort - we can clean up stored images later
            }

            return Ok(HttpResponse::Ok().finish());
        }
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().body("unable to create review pic"));
        }
    }
}
