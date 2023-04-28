use crate::{
    authorization::AuthenticatedUser, db::get_reviews_from_bounds_with_exclusions,
    review_v1::ReviewAnnotationPub,
};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Json, Query, ReqData},
    Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct ReviewMapBoundWithExclusionRequest {
    latitude_north: f64,
    latitude_south: f64,
    longitude_west: f64,
    longitude_east: f64,
    latitude_north_e: f64,
    latitude_south_e: f64,
    longitude_west_e: f64,
    longitude_east_e: f64,
    page: u32,
}

/// Gets reviews you are able to see if a given map bounding box.
#[get("/reviews_from_bounds_exclusions")]
pub async fn get_reviews_from_map_bounds_with_exclusions(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    review_location_request: Query<ReviewMapBoundWithExclusionRequest>,
) -> Result<impl Responder> {
    let review_res = get_reviews_from_bounds_with_exclusions(
        &pool,
        &authenticated_user.0,
        review_location_request.latitude_north,
        review_location_request.latitude_south,
        review_location_request.longitude_west,
        review_location_request.longitude_east,
        review_location_request.latitude_north_e,
        review_location_request.latitude_south_e,
        review_location_request.longitude_west_e,
        review_location_request.longitude_east_e,
        review_location_request.page,
    )
    .await;

    match review_res {
        Ok(reviews) => {
            let reviews_pub: Vec<ReviewAnnotationPub> = reviews
                .into_iter()
                .map(|f| -> ReviewAnnotationPub { f.into() })
                .collect();
            Ok(Json(reviews_pub))
        }
        Err(err) => return Err(ErrorInternalServerError(err.to_string())),
    }
}
