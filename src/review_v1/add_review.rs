use crate::{
    authorization::AuthenticatedUser,
    db::{create_pic, create_review, remove_review_and_children, remove_review_pic_id, Review},
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::{Data, Json, ReqData},
    HttpResponse, Responder, Result,
};
use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use images::{ByteStream, PutObjectRequest, S3Client, S3};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;
use validation::{
    validate_latitude, validate_location_name, validate_longitude, validate_review_category,
    validate_review_pic_b64, validate_review_text, validate_stars,
};

use super::review_types::ReviewPub;

#[derive(Deserialize, Serialize)]
pub struct AddReviewRequest {
    pub text: String,
    pub stars: u8,
    pub category: String,
    pub location_name: String,
    pub pic: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_custom: bool,
}

/// Allows the user to create a review for a specific place.
#[post("/")]
pub async fn add_review(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    s3_client: Data<S3Client>,
    add_review_request: Json<AddReviewRequest>,
) -> Result<impl Responder> {
    if let Err(err) = validate_review_text(&add_review_request.text) {
        return Err(ErrorBadRequest(err.to_string()));
    }

    if let Err(err) = validate_longitude(add_review_request.longitude) {
        return Err(ErrorBadRequest(err.to_string()));
    }

    if let Err(err) = validate_latitude(add_review_request.latitude) {
        return Err(ErrorBadRequest(err.to_string()));
    }

    if let Err(err) = validate_location_name(&add_review_request.location_name) {
        return Err(ErrorBadRequest(err.to_string()));
    }

    if let Err(err) = validate_stars(add_review_request.stars) {
        return Err(ErrorBadRequest(err.to_string()));
    }

    if let Err(err) = validate_review_category(&add_review_request.category) {
        return Err(ErrorBadRequest(err.to_string()));
    }

    let validation_result = validate_review_pic_b64(&add_review_request.pic);

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

    let review = map_review_to_db(&add_review_request, &authenticated_user.0);

    let create_res = create_review(&pool, &review).await;

    match create_res {
        Ok(_) => {
            let pic_res = create_pic(&pool, Some(review.id.clone()), width, height).await;

            match pic_res {
                Ok(pic) => {
                    if let Ok(bytes) = general_purpose::STANDARD.decode(&add_review_request.pic) {
                        if let Err(_) = s3_client
                            .put_object(PutObjectRequest {
                                body: Some(ByteStream::from(bytes)),
                                bucket: "bout".to_string(),
                                key: pic.id.clone(),
                                acl: Some("public-read".to_string()),
                                ..Default::default()
                            })
                            .await
                        {
                            let _ = remove_review_pic_id(&pool, &pic.id, &review.id).await;
                            let _ = remove_review_and_children(&pool, &review.id).await;

                            return Ok(HttpResponse::InternalServerError()
                                .body("unable to store review pic"));
                        }

                        return Ok(HttpResponse::Ok().json(ReviewPub::from(review)));
                    } else {
                        let _ = remove_review_pic_id(&pool, &pic.id, &review.id).await;
                        let _ = remove_review_and_children(&pool, &review.id).await;

                        return Ok(
                            HttpResponse::InternalServerError().body("failed to read pic buffer")
                        );
                    }
                }
                Err(_) => {
                    return Ok(
                        HttpResponse::InternalServerError().body("unable to create review pic")
                    );
                }
            }
        }
        Err(_) => {
            return Err(ErrorInternalServerError("failed to create review"));
        }
    }
}

fn map_review_to_db(request: &AddReviewRequest, user_id: &str) -> Review {
    Review {
        id: Uuid::new_v4().to_string(),
        user_id: user_id.to_string(),
        created: Utc::now().naive_utc(),
        pic_id: None,
        category: request.category.clone(),
        text: request.text.clone(),
        stars: request.stars,
        location_name: request.location_name.clone(),
        latitude: request.latitude,
        longitude: request.longitude,
        is_custom: request.is_custom,
    }
}
