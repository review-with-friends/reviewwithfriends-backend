use crate::{
    authorization::AuthenticatedUser,
    db::{get_incoming_friend_requests, ignore_friend_request},
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
pub struct IgnoreRequest {
    request_id: String,
}

/// Allows you to ignore a friend request.
/// The sending user won't be able to send another.
#[post("/ignore_friend")]
pub async fn ignore_friend(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    ignore_request: Query<IgnoreRequest>,
) -> Result<impl Responder> {
    let friend_requests_res =
        get_incoming_friend_requests(&pool, &authenticated_user.0.clone()).await;

    match friend_requests_res {
        Ok(friend_requests) => {
            let friend_request_exists = friend_requests.into_iter().any(|fr| -> bool {
                return fr.id == ignore_request.request_id;
            });

            if friend_request_exists {
                let ignore_res = ignore_friend_request(
                    &pool,
                    &ignore_request.request_id,
                    &&authenticated_user.0.clone(),
                )
                .await;

                match ignore_res {
                    Ok(_) => Ok(HttpResponse::Ok()),
                    Err(_) => Err(ErrorInternalServerError("failed ignoring friend request")),
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
