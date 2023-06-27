use crate::{
    authorization::AuthenticatedUser, db::get_review, review_v1::gather_compound_review,
    tracing::add_error_span,
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get,
    web::{Data, Json, Query, ReqData},
    Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

use super::review_types::ReviewPub;

#[derive(Deserialize)]
pub struct ReviewRequest {
    review_id: String,
}

/// Gets a review by the given id.
/// The returned object contains all
/// the initial information for the review.
#[get("/review_by_id")]
pub async fn get_review_by_id(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    review_request: Query<ReviewRequest>,
) -> Result<impl Responder> {
    let review: ReviewPub;
    let review_res = get_review(&pool, &authenticated_user.0, &review_request.review_id).await;

    match review_res {
        Ok(review_opt) => {
            if let Some(review_tmp) = review_opt {
                review = review_tmp.into();
            } else {
                return Err(ErrorNotFound("unable to find review"));
            }
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError("unable to get review"));
        }
    }

    let compound_review_res = gather_compound_review(&pool, &authenticated_user.0, review).await;

    match compound_review_res {
        Ok(compound_review) => {
            return Ok(Json(compound_review));
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError(
                "failed to gather review components",
            ));
        }
    }
}
