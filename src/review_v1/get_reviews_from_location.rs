use crate::{authorization::AuthenticatedUser, db::get_reviews_from_location};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Json, Query, ReqData},
    Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

use super::review_types::ReviewPub;

#[derive(Deserialize)]
pub struct ReviewLocationRequest {
    latitude: f64,
    longitude: f64,
    name: String,
}

/// Gets reviews you are able to see that qualify via close location.
/// Accuracy required is exact for location name; but +- a range around
/// the given coordinates.
#[get("/reviews_from_loc")]
pub async fn get_reviews_from_loc(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    review_location_request: Query<ReviewLocationRequest>,
) -> Result<impl Responder> {
    let review_res = get_reviews_from_location(
        &pool,
        &authenticated_user.0,
        &review_location_request.name,
        review_location_request.latitude,
        review_location_request.longitude,
    )
    .await;

    match review_res {
        Ok(reviews) => {
            let reviews_pub: Vec<ReviewPub> = reviews
                .into_iter()
                .map(|f| -> ReviewPub { f.into() })
                .collect();
            Ok(Json(reviews_pub))
        }
        Err(_) => {
            return Err(ErrorInternalServerError(
                "unable to get reviews for location".to_string(),
            ))
        }
    }
}
