use crate::{
    authorization::AuthenticatedUser, compound_types::CompoundReviewPub, db::get_latest_reviews,
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
pub struct ReviewLatestRequest {
    page: u32,
}

/// Gets the latest reviews available to a given requesting user.
/// Additionally resolves the 'fullreview' which includes likes, replies, and pic records.
#[get("/latest_full")]
pub async fn get_latest_full(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    review_latest_request: Query<ReviewLatestRequest>,
) -> Result<impl Responder> {
    let review_res =
        get_latest_reviews(&pool, &authenticated_user.0, review_latest_request.page).await;

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
                "unable to get latest reviews".to_string(),
            ));
        }
    }

    return Ok(Json(compound_reviews));
}
