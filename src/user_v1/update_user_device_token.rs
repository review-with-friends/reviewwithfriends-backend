use crate::{authorization::AuthenticatedUser, db::update_device_token};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Query, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct UpdateUserDeviceTokenRequest {
    device_token: String,
}

/// Allows the updating of display_name and name fields.
/// display_name isn't unique in the table; but name is.
#[post("/device_token")]
pub async fn update_user_device_token(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    update_request: Query<UpdateUserDeviceTokenRequest>,
) -> Result<impl Responder> {
    let update_res =
        update_device_token(&pool, &authenticated_user.0, &update_request.device_token).await;

    match update_res {
        Ok(_) => return Ok(HttpResponse::Ok().finish()),
        Err(_) => return Err(ErrorInternalServerError("failed to update user")),
    }
}
