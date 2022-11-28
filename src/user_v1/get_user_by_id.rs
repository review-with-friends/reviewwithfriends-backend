use crate::{authorization::AuthenticatedUser, db::get_user};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get,
    web::{Data, Json, Query, ReqData},
    Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

use super::user_types::UserPub;

#[derive(Deserialize)]
pub struct GetUserByIdRequest {
    id: String,
}

#[get("/by_id")]
pub async fn get_user_by_id(
    _authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    get_user_request: Query<GetUserByIdRequest>,
) -> Result<impl Responder> {
    let user_res = get_user(&pool, &get_user_request.id).await;

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
