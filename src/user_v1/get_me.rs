use crate::{authorization::AuthenticatedUser, db::get_user};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get,
    web::{Data, Json, ReqData},
    Responder, Result,
};
use sqlx::MySqlPool;

use super::user_types::UserPub;

/// Fetches your own user record.
#[get("/me")]
pub async fn get_me(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
) -> Result<impl Responder> {
    let user_res = get_user(&pool, &authenticated_user.0).await;

    match user_res {
        Ok(user_opt) => {
            if let Some(user) = user_opt {
                return Ok(Json(UserPub::from(user)));
            } else {
                return Err(ErrorNotFound("could not find user"));
            }
        }
        Err(_) => return Err(ErrorInternalServerError("unable to get user".to_string())),
    }
}
