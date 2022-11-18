use crate::{
    authorization::AuthenticatedUser,
    db::{accept_friend_request, get_incoming_friend_requests},
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
pub struct AcceptRequest {
    request_id: String,
}

#[post("/accept_friend")]
pub async fn accept_friend(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    accept_request: Query<AcceptRequest>,
) -> Result<impl Responder> {
    let friend_requests_res =
        get_incoming_friend_requests(&pool, &authenticated_user.0.clone()).await;

    match friend_requests_res {
        Ok(friend_requests) => {
            let request_opt = friend_requests.into_iter().find(|fr| -> bool {
                return fr.id == accept_request.request_id;
            });

            match request_opt {
                Some(request) => {
                    let accept_res = accept_friend_request(
                        &pool,
                        &&authenticated_user.0.clone(),
                        &request.user_id,
                    )
                    .await;

                    match accept_res {
                        Ok(_) => Ok(HttpResponse::Ok()),
                        Err(_) => Err(ErrorInternalServerError("failed accepting friend request")),
                    }
                }
                None => {
                    return Err(ErrorBadRequest("friend request doesnt exist"));
                }
            }
        }
        Err(_) => {
            return Err(ErrorInternalServerError(
                "could not fetch incoming friend requests",
            ))
        }
    }
}
