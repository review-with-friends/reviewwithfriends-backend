use crate::{
    authorization::AuthenticatedUser, compound_types::CompoundReviewPub, db,
    review_v1::gather_compound_review, tracing::add_error_span,
};
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
    page: u32,
}

#[get("/recommended_reviews_from_user")]
pub async fn get_recommended_reviews_from_user(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    user_review_request: Query<UserReviewRequest>,
) -> Result<impl Responder> {
    let review_res = db::get_recommended_reviews_from_user(
        &pool,
        &authenticated_user.0,
        &user_review_request.user_id,
        user_review_request.page,
    )
    .await;

    let mut compound_reviews: Vec<CompoundReviewPub> = vec![];

    match review_res {
        Ok(reviews) => {
            let reviews_pub: Vec<ReviewPub> = reviews
                .into_iter()
                .map(|f| -> ReviewPub { f.into() })
                .collect();

            for review_pub in reviews_pub.into_iter() {
                let compound_review_res = gather_compound_review(&pool, review_pub).await;

                match compound_review_res {
                    Ok(compound_review) => compound_reviews.push(compound_review),
                    Err(error) => {
                        add_error_span(&error);
                        return Err(ErrorInternalServerError(
                            "failed gathering review contents".to_string(),
                        ));
                    }
                }
            }
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError(
                "unable to get recommended reviews for user".to_string(),
            ));
        }
    }

    return Ok(Json(compound_reviews));
}
