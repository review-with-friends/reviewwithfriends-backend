use crate::{
    authorization::AuthenticatedUser,
    db::{cancel_friend_request, get_outgoing_friend_requests},
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
pub struct CancelRequest {
    request_id: String,
}

#[post("/cancel_friend")]
pub async fn cancel_friend(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    cancel_request: Query<CancelRequest>,
) -> Result<impl Responder> {
    let friend_requests_res =
        get_outgoing_friend_requests(&pool, &authenticated_user.0.clone()).await;

    match friend_requests_res {
        Ok(friend_requests) => {
            let friend_request_exists = friend_requests.into_iter().any(|fr| -> bool {
                return fr.id == cancel_request.request_id;
            });

            if friend_request_exists {
                let cancel_res = cancel_friend_request(
                    &pool,
                    &cancel_request.request_id,
                    &&authenticated_user.0.clone(),
                )
                .await;

                match cancel_res {
                    Ok(_) => Ok(HttpResponse::Ok()),
                    Err(_) => Err(ErrorInternalServerError("failed cancelling friend request")),
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
