use crate::{authorization::AuthenticatedUser, db::get_latest_reviews, tracing::add_error_span};
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
    page: u32,
}

/// Simple activity tracking.
/// Paged for infinite scroll support.
#[get("/latest")]
pub async fn get_latest(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    review_location_request: Query<ReviewLocationRequest>,
) -> Result<impl Responder> {
    let review_res =
        get_latest_reviews(&pool, &authenticated_user.0, review_location_request.page).await;

    match review_res {
        Ok(reviews) => {
            let reviews_pub: Vec<ReviewPub> = reviews
                .into_iter()
                .map(|f| -> ReviewPub { f.into() })
                .collect();
            Ok(Json(reviews_pub))
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError(
                "unable to get latest reviews".to_string(),
            ));
        }
    }
}
