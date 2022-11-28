use crate::{authorization::AuthenticatedUser, db::get_outgoing_friend_requests};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Json, ReqData},
    Responder, Result,
};
use sqlx::MySqlPool;

use super::friend_types::FriendRequestPub;

/// Gets a users outgoing friend request.
#[get("/outgoing_friends")]
pub async fn get_outgoing_friends(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
) -> Result<impl Responder> {
    let friend_requests_res = get_outgoing_friend_requests(&pool, &authenticated_user.0).await;

    match friend_requests_res {
        Ok(friend_requests) => {
            let friend_requests_pub: Vec<FriendRequestPub> = friend_requests
                .into_iter()
                .map(|f| -> FriendRequestPub { f.into() })
                .collect();
            Ok(Json(friend_requests_pub))
        }
        Err(_) => {
            return Err(ErrorInternalServerError(
                "could not fetch outgoing friend requests",
            ))
        }
    }
}
