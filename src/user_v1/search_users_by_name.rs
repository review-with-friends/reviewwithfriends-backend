use crate::{authorization::AuthenticatedUser, db::search_user_from_name};
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
pub struct UserSearchRequest {
    name: String,
}

/// Searches for users by name.
/// Returns a list of the results.
#[get("/search_by_name")]
pub async fn search_user_by_name(
    _authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    search_request: Query<UserSearchRequest>,
) -> Result<impl Responder> {
    let search_result_res = search_user_from_name(&pool, &search_request.name).await;

    match search_result_res {
        Ok(results) => {
            let friend_requests_pub: Vec<UserPub> = results
                .into_iter()
                .map(|f| -> UserPub { f.into() })
                .collect();
            Ok(Json(friend_requests_pub))
        }
        Err(_) => return Err(ErrorInternalServerError("could not complete user search")),
    }
}
