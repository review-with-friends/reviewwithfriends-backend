use crate::{authorization::AuthenticatedUser, db, tracing::add_error_span};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Query, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct RemoveBookmarkRequest {
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
}

/// Gets all the replies for a given review.
#[post("/remove_bookmark")]
pub async fn remove_bookmark(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    remove_bookmark_request: Query<RemoveBookmarkRequest>,
) -> Result<impl Responder> {
    let remove_bookmark_res = db::remove_bookmark(
        &pool,
        &authenticated_user.0,
        &remove_bookmark_request.location_name,
        remove_bookmark_request.latitude,
        remove_bookmark_request.longitude,
    )
    .await;

    match remove_bookmark_res {
        Ok(_) => {
            return Ok(HttpResponse::Ok().finish());
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError(
                "unable to remove bookmark".to_string(),
            ));
        }
    }
}
