use super::friend_types::FriendPub;
use crate::{
    authorization::AuthenticatedUser,
    db::{are_users_friends, get_current_friends},
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    get,
    web::{Data, Json, Query, ReqData},
    Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct GetUserFriendRequest {
    user_id: String,
}

/// Allows a user to get a friends list of a user they are friends with.
#[get("user")]
pub async fn get_user_friends(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    user_friends_query: Query<GetUserFriendRequest>,
) -> Result<impl Responder> {
    let are_friends_res =
        are_users_friends(&pool, &authenticated_user.0, &user_friends_query.user_id).await;

    match are_friends_res {
        Ok(are_friends) => {
            if are_friends {
                let friends_res = get_current_friends(&pool, &user_friends_query.user_id).await;
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
            } else {
                return Err(ErrorBadRequest("you are not friends with this user"));
            }
        }
        Err(_) => {
            return Err(ErrorInternalServerError(
                "failed to validate if user is friend",
            ))
        }
    }
}
