use crate::{authorization::AuthenticatedUser, db};
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
pub struct UserReviewRequest {
    user_id: String,
}

/// Gets reviews you are able to see that qualify via close location.
/// Accuracy required is exact for location name; but +- a range around
/// the given coordinates.
#[get("/reviews_from_user")]
pub async fn get_reviews_from_user(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    user_review_request: Query<UserReviewRequest>,
) -> Result<impl Responder> {
    let review_res =
        db::get_reviews_from_user(&pool, &authenticated_user.0, &user_review_request.user_id).await;

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
