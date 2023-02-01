use crate::{
    authorization::AuthenticatedUser,
    db::{create_review, Review},
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::{Data, Json, ReqData},
    HttpResponse, Responder, Result,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;
use validation::{
    validate_latitude, validate_location_name, validate_longitude, validate_review_category,
    validate_review_text, validate_stars,
};

use super::review_types::ReviewPub;

#[derive(Deserialize, Serialize)]
pub struct AddReviewRequest {
    pub text: String,
    pub stars: u8,
    pub category: String,
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_custom: bool,
}

/// Allows the user to create a review for a specific place.
#[post("/")]
pub async fn add_review(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
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

    // TODO: Validate incoming things are within the range.
    let review = map_review_to_db(&add_review_request, &authenticated_user.0);

    let create_res = create_review(&pool, &review).await;

    match create_res {
        Ok(_) => {
            return Ok(HttpResponse::Ok().json(ReviewPub::from(review)));
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
