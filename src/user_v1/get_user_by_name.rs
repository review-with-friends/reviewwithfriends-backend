use crate::{authorization::AuthenticatedUser, db::get_user_from_name};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Json, Query, ReqData},
    Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

use super::user_types::UserPub;

#[derive(Deserialize)]
pub struct GetUserByNameRequest {
    name: String,
}

#[get("/by_name")]
pub async fn get_user_by_name(
    _authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    get_user_request: Query<GetUserByNameRequest>,
) -> Result<impl Responder> {
    let user_res = get_user_from_name(&pool, &get_user_request.name).await;

    match user_res {
        Ok(user) => Ok(Json(UserPub::from(user))),
        Err(_) => return Err(ErrorInternalServerError("unable to get user".to_string())),
    }
}
