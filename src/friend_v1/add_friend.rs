use crate::{
    authorization::AuthenticatedUser,
    db::{
        create_friend_request, does_user_exist, get_current_friends, get_outgoing_friend_requests,
    },
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
pub struct SendRequest {
    friend_id: String,
}

/// Allows users to send a friend request to another user.
#[post("/add_friend")]
pub async fn add_friend(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    send_request: Query<SendRequest>,
) -> Result<impl Responder> {
    if &authenticated_user.0 == &send_request.friend_id {
        return Err(ErrorBadRequest("you cant add yourself"));
    }

    let exists_res = does_user_exist(&pool, &send_request.friend_id).await;
    match exists_res {
        Ok(exists) => {
            if !exists {
                return Err(ErrorBadRequest("no user exists with that id"));
            }
        }
        Err(_) => {
            return Err(ErrorBadRequest("unable to get user"));
        }
    }

    let existing_requests_res =
        get_outgoing_friend_requests(&pool, &authenticated_user.0.clone()).await;
    match existing_requests_res {
        Ok(existing_requests) => {
            if existing_requests
                .into_iter()
                .any(|er| -> bool { &er.friend_id == &send_request.friend_id })
            {
                return Err(ErrorBadRequest("friend request already sent"));
            }

            let friends_res = get_current_friends(&pool, &authenticated_user.0.clone()).await;

            match friends_res {
                Ok(friends) => {
                    if friends
                        .into_iter()
                        .any(|f| -> bool { &f.friend_id == &send_request.friend_id })
                    {
                        return Err(ErrorBadRequest("already friends"));
                    }

                    let create_res = create_friend_request(
                        &pool,
                        &authenticated_user.0.clone().as_str(),
                        &send_request.friend_id,
                    )
                    .await;
                    match create_res {
                        Ok(_) => return Ok(HttpResponse::Ok()),
                        Err(_) => {
                            return Err(ErrorInternalServerError("could not create friend request"))
                        }
                    }
                }
                Err(_) => return Err(ErrorInternalServerError("unable to fetch friends")),
            }
        }
        Err(_) => {
            return Err(ErrorInternalServerError(
                "unable to fetch existing requests",
            ))
        }
    }
}
