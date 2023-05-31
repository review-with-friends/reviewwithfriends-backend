use crate::{authorization::AuthenticatedUser, db, db::get_user, tracing::add_error_span};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::{Data, Query, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct UserReportRequest {
    user_id: String,
}

/// Report a user.
#[post("/user")]
pub async fn report_user(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    user_report_request: Query<UserReportRequest>,
) -> Result<impl Responder> {
    let user_res = get_user(&pool, &user_report_request.user_id).await;

    match user_res {
        Ok(target_user_opt) => {
            if target_user_opt.is_none() {
                return Err(ErrorBadRequest("target user doesn't exist".to_string()));
            }
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError(
                "unable to get target user".to_string(),
            ));
        }
    }

    let report_res =
        db::report_user(&pool, &user_report_request.user_id, &authenticated_user.0).await;
    match report_res {
        Ok(_) => {
            return Ok(HttpResponse::Ok().finish());
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError(
                "unable to create report".to_string(),
            ));
        }
    }
}
