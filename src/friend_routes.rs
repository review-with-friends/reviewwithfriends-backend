use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    get, post,
    web::{Data, Json, Query, ReqData},
    HttpResponse, Responder, Result,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::{
    authorization::AuthenticatedUser,
    db::{
        accept_friend_request, cancel_friend_request, create_friend_request,
        decline_friend_request, does_user_exist, get_current_friends, get_incoming_friend_requests,
        get_incoming_ignored_friend_requests, get_outgoing_friend_requests, ignore_friend_request,
        remove_current_friend, Friend, FriendRequest,
    },
};

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
struct FriendPub {
    pub id: String,
    pub created: NaiveDateTime,
    pub user_id: String,
    pub friend_id: String,
}

impl From<Friend> for FriendPub {
    fn from(friend: Friend) -> FriendPub {
        FriendPub {
            id: friend.id,
            created: friend.created,
            user_id: friend.user_id,
            friend_id: friend.friend_id,
        }
    }
}

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
struct FriendRequestPub {
    pub id: String,
    pub created: NaiveDateTime,
    pub user_id: String,
    pub friend_id: String,
}

impl From<FriendRequest> for FriendRequestPub {
    fn from(friend_request: FriendRequest) -> FriendRequestPub {
        FriendRequestPub {
            id: friend_request.id,
            created: friend_request.created,
            user_id: friend_request.user_id,
            friend_id: friend_request.friend_id,
        }
    }
}

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

#[get("/outgoing_requests")]
pub async fn get_outgoing_requests(
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

#[get("/incoming_requests")]
pub async fn get_incoming_requests(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
) -> Result<impl Responder> {
    let friend_requests_res = get_incoming_friend_requests(&pool, &authenticated_user.0).await;

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
                "could not fetch incoming friend requests",
            ))
        }
    }
}

#[get("/incoming_ignored_requests")]
pub async fn get_incoming_ignored_requests(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
) -> Result<impl Responder> {
    let friend_requests_res =
        get_incoming_ignored_friend_requests(&pool, &authenticated_user.0).await;

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
                "could not fetch incoming ignored friend requests",
            ))
        }
    }
}

#[derive(Deserialize)]
pub struct SendRequest {
    friend_id: String,
}

#[post("/send_request")]
pub async fn send_request(
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

#[derive(Deserialize)]
pub struct AcceptRequest {
    request_id: String,
}

#[post("/accept_request")]
pub async fn accept_request(
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

#[derive(Deserialize)]
pub struct CancelRequest {
    request_id: String,
}

#[post("/cancel_request")]
pub async fn cancel_request(
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

#[derive(Deserialize)]
pub struct IgnoreRequest {
    request_id: String,
}

#[post("/ignore_request")]
pub async fn ignore_request(
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

#[derive(Deserialize)]
pub struct DeclineRequest {
    request_id: String,
}

#[post("/decline_request")]
pub async fn decline_request(
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

#[derive(Deserialize)]
pub struct RemoveRequest {
    friend_id: String,
}

#[post("/remove")]
pub async fn remove_friend(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    remove_request: Query<RemoveRequest>,
) -> Result<impl Responder> {
    let friends_res = get_current_friends(&pool, &authenticated_user.0.clone()).await;

    match friends_res {
        Ok(friends) => {
            let friend_exists = friends.into_iter().any(|fr| -> bool {
                return fr.friend_id == remove_request.friend_id;
            });

            if friend_exists {
                let remove_res = remove_current_friend(
                    &pool,
                    &&authenticated_user.0.clone(),
                    &remove_request.friend_id,
                )
                .await;

                match remove_res {
                    Ok(_) => Ok(HttpResponse::Ok()),
                    Err(_) => Err(ErrorInternalServerError("failed removing friend")),
                }
            } else {
                return Err(ErrorBadRequest("friend doesnt exist"));
            }
        }
        Err(_) => return Err(ErrorInternalServerError("could not fetch friends")),
    }
}
