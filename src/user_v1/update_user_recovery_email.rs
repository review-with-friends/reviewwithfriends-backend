use crate::{authorization::AuthenticatedUser, db::update_recovery_email};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::{Data, Query, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct UpdateUserRecoveryEmailRequest {
    recovery_email: String,
}

#[post("/recovery_email")]
pub async fn update_user_recovery_email(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    update_request: Query<UpdateUserRecoveryEmailRequest>,
) -> Result<impl Responder> {
    let valid_email = validation::validate_email(&update_request.recovery_email);
    if let Err(email_err) = valid_email {
        return Err(ErrorBadRequest(email_err.to_string()));
    }

    let update_res =
        update_recovery_email(&pool, &authenticated_user.0, &update_request.recovery_email).await;

    match update_res {
        Ok(_) => return Ok(HttpResponse::Ok().finish()),
        Err(_) => return Err(ErrorInternalServerError("failed to update user")),
    }
}
