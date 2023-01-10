use crate::{
    authorization::AuthenticatedUser,
    db::{
        get_current_friends, get_incoming_friend_requests, get_incoming_ignored_friend_requests,
        get_outgoing_friend_requests,
    },
};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Json, ReqData},
    Responder, Result,
};
use sqlx::MySqlPool;

use super::{friend_types::FriendPub, FriendRequestPub, FullFriendsPub};

/// Allows a user to get their friends list.
#[get("/full_friends")]
pub async fn full_friends(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
) -> Result<impl Responder> {
    let friends_res = get_current_friends(&pool, &authenticated_user.0).await;

    let friends: Vec<FriendPub>;
    match friends_res {
        Ok(friends_tmp) => {
            friends = friends_tmp
                .into_iter()
                .map(|f| -> FriendPub { f.into() })
                .collect();
        }
        Err(_) => return Err(ErrorInternalServerError("could not get friends")),
    }

    let ignored_requests_res =
        get_incoming_ignored_friend_requests(&pool, &authenticated_user.0).await;

    let ignored_requests: Vec<FriendRequestPub>;
    match ignored_requests_res {
        Ok(friend_requests) => {
            ignored_requests = friend_requests
                .into_iter()
                .map(|f| -> FriendRequestPub { f.into() })
                .collect();
        }
        Err(_) => {
            return Err(ErrorInternalServerError(
                "could not fetch incoming ignored friend requests",
            ))
        }
    }

    let incoming_requests_res = get_incoming_friend_requests(&pool, &authenticated_user.0).await;

    let incoming_requests: Vec<FriendRequestPub>;
    match incoming_requests_res {
        Ok(friend_requests) => {
            incoming_requests = friend_requests
                .into_iter()
                .map(|f| -> FriendRequestPub { f.into() })
                .collect();
        }
        Err(_) => {
            return Err(ErrorInternalServerError(
                "could not fetch incoming friend requests",
            ))
        }
    }

    let outgoing_requests_res = get_outgoing_friend_requests(&pool, &authenticated_user.0).await;

    let outgoing_requests: Vec<FriendRequestPub>;
    match outgoing_requests_res {
        Ok(friend_requests) => {
            outgoing_requests = friend_requests
                .into_iter()
                .map(|f| -> FriendRequestPub { f.into() })
                .collect();
        }
        Err(_) => {
            return Err(ErrorInternalServerError(
                "could not fetch outgoing friend requests",
            ))
        }
    }

    Ok(Json(FullFriendsPub {
        friends,
        incoming_requests,
        outgoing_requests,
        ignored_requests,
    }))
}
