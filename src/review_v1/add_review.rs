use crate::{
    authorization::AuthenticatedUser,
    db::{create_review, Review},
};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Json, ReqData},
    HttpResponse, Responder, Result,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;

use super::review_types::ReviewPub;

#[derive(Deserialize, Serialize)]
pub struct AddReviewRequest {
    pub created: NaiveDateTime,
    pub pic_id: Option<String>,
    pub category: String,
    pub text: String,
    pub stars: u8,
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_custom: bool,
}

#[post("/")]
pub async fn add_review(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    add_review_request: Json<AddReviewRequest>,
) -> Result<impl Responder> {
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
        created: request.created,
        pic_id: request.pic_id.clone(),
        category: request.category.clone(),
        text: request.text.clone(),
        stars: request.stars,
        location_name: request.location_name.clone(),
        latitude: request.latitude,
        longitude: request.longitude,
        is_custom: request.is_custom,
    }
}
