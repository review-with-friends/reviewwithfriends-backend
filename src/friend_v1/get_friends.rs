use crate::{authorization::AuthenticatedUser, db::get_current_friends};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Json, ReqData},
    Responder, Result,
};
use sqlx::MySqlPool;

use super::friend_types::FriendPub;

/// Allows a user to get their friends list.
#[get("")]
pub async fn get_friends(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
) -> Result<impl Responder> {
    let friends_res = get_current_friends(&pool, &authenticated_user.0).await;

    match friends_res {
        Ok(friends) => {
            let friends_pub: Vec<FriendPub> = friends
                .into_iter()
                .map(|f| -> FriendPub { f.into() })
                .collect();
            Ok(Json(friends_pub))
        }
        Err(_) => return Err(ErrorInternalServerError("could not get friends")),
    }
}
