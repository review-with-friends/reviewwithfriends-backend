use crate::{
    authorization::AuthenticatedUser,
    db::{decline_friend_request, get_incoming_friend_requests},
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::{Data, Query, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct DeclineRequest {
    request_id: String,
}

/// Allows users to decline an incoming friend request.
#[post("/decline_friend")]
pub async fn decline_friend(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    decline_request: Query<DeclineRequest>,
) -> Result<impl Responder> {
    let friend_requests_res =
        get_incoming_friend_requests(&pool, &authenticated_user.0.clone()).await;

    match friend_requests_res {
        Ok(friend_requests) => {
            let friend_request_exists = friend_requests.into_iter().any(|fr| -> bool {
                return fr.id == decline_request.request_id;
            });

            if friend_request_exists {
                let ignore_res = decline_friend_request(
                    &pool,
                    &decline_request.request_id,
                    &&authenticated_user.0.clone(),
                )
                .await;

                match ignore_res {
                    Ok(_) => Ok(HttpResponse::Ok()),
                    Err(_) => Err(ErrorInternalServerError("failed declining friend request")),
                }
            } else {
                return Err(ErrorBadRequest("friend request doesnt exist"));
            }
        }
        Err(_) => {
            return Err(ErrorInternalServerError(
                "could not fetch incoming friend requests",
            ))
        }
    }
}
