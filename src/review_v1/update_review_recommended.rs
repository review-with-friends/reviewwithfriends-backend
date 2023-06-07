use crate::{
    authorization::AuthenticatedUser, db::update_review_recommended, tracing::add_error_span,
};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Query, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct UpdateReviewRecommended {
    review_id: String,
    recommended: bool,
}

#[post("/update_recommended")]
pub async fn update_review_recommended_status(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    update_request: Query<UpdateReviewRecommended>,
) -> Result<impl Responder> {
    let update_res = update_review_recommended(
        &pool,
        &update_request.review_id,
        &authenticated_user.0,
        update_request.recommended,
    )
    .await;

    match update_res {
        Ok(_) => return Ok(HttpResponse::Ok().finish()),
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError(
                "failed to update recommended status",
            ));
        }
    }
}
