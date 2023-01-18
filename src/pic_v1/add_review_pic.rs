use crate::{
    authorization::AuthenticatedUser,
    db::{create_pic, get_all_pics, get_review, remove_review_pic_id, Review},
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
    let validation_result = validate_review_pic(&pic_bytes);

    let width: u16;
    let height: u16;

    match validation_result {
        Ok(size) => {
            width = size.0;
            height = size.1;
        }
        Err(err) => {
            return Ok(HttpResponse::BadRequest().body(err));
        }
    }

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

    let pics_res = get_all_pics(&pool, &review.id).await;

    match pics_res {
        Ok(pics) => {
            if pics.len() >= 4 {
                return Ok(HttpResponse::BadRequest().body("too many pics already"));
            }
        }
        Err(_) => return Ok(HttpResponse::BadRequest().body("unable to get pics")),
    }

    let pic_res = create_pic(
        &pool,
        Some(add_review_pic_request.review_id.clone()),
        width,
        height,
    )
    .await;

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
                let _ = remove_review_pic_id(&pool, &pic.id, &review.id).await;

                return Ok(HttpResponse::InternalServerError().body("unable to store review pic"));
            }

            return Ok(HttpResponse::Ok().finish());
        }
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().body("unable to create review pic"));
        }
    }
}
