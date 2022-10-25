use chrono::NaiveDateTime;
use rocket::{http::Status, response::status::Custom, serde::json::serde_json};
use serde::Serialize;

use crate::{
    db::{
        accept_friend_request, cancel_friend_request, create_friend_request,
        decline_friend_request, does_user_exist, get_current_friends, get_incoming_friend_requests,
        get_incoming_ignored_friend_requests, get_outgoing_friend_requests, ignore_friend_request,
        remove_current_friend, DBClient, Friend, FriendRequest,
    },
    JWTAuthorized,
};

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropraitley.
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
/// to trim database object appropraitley.
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

#[get("/")]
pub async fn get_friends(auth: JWTAuthorized, client: &DBClient) -> Result<String, Custom<String>> {
    let friends_res = get_current_friends(client, auth.0).await;

    match friends_res {
        Ok(friends) => {
            let friends_pub: Vec<FriendPub> = friends
                .into_iter()
                .map(|f| -> FriendPub { f.into() })
                .collect();
            Ok(serde_json::to_string(&friends_pub).unwrap())
        }
        Err(_) => {
            return Err(Custom(
                Status::InternalServerError,
                "could not fetch friends".to_string(),
            ))
        }
    }
}

#[get("/outgoing_requests")]
pub async fn get_outgoing_requests(
    auth: JWTAuthorized,
    client: &DBClient,
) -> Result<String, Custom<String>> {
    let friend_requests_res = get_outgoing_friend_requests(client, auth.0).await;

    match friend_requests_res {
        Ok(friend_requests) => {
            let friend_requests_pub: Vec<FriendRequestPub> = friend_requests
                .into_iter()
                .map(|f| -> FriendRequestPub { f.into() })
                .collect();
            Ok(serde_json::to_string(&friend_requests_pub).unwrap())
        }
        Err(_) => {
            return Err(Custom(
                Status::InternalServerError,
                "could not fetch outgoing friend requests".to_string(),
            ))
        }
    }
}

#[get("/incoming_requests")]
pub async fn get_incoming_requests(
    auth: JWTAuthorized,
    client: &DBClient,
) -> Result<String, Custom<String>> {
    let friend_requests_res = get_incoming_friend_requests(client, auth.0).await;

    match friend_requests_res {
        Ok(friend_requests) => {
            let friend_requests_pub: Vec<FriendRequestPub> = friend_requests
                .into_iter()
                .map(|f| -> FriendRequestPub { f.into() })
                .collect();
            Ok(serde_json::to_string(&friend_requests_pub).unwrap())
        }
        Err(_) => {
            return Err(Custom(
                Status::InternalServerError,
                "could not fetch incoming friend requests".to_string(),
            ))
        }
    }
}

#[get("/incoming_ignored_requests")]
pub async fn get_incoming_ignored_requests(
    auth: JWTAuthorized,
    client: &DBClient,
) -> Result<String, Custom<String>> {
    let friend_requests_res = get_incoming_ignored_friend_requests(client, auth.0).await;

    match friend_requests_res {
        Ok(friend_requests) => {
            let friend_requests_pub: Vec<FriendRequestPub> = friend_requests
                .into_iter()
                .map(|f| -> FriendRequestPub { f.into() })
                .collect();
            Ok(serde_json::to_string(&friend_requests_pub).unwrap())
        }
        Err(_) => {
            return Err(Custom(
                Status::InternalServerError,
                "could not fetch incoming ignored friend requests".to_string(),
            ))
        }
    }
}

#[post("/send_request?<friend_id>")]
pub async fn send_request(
    auth: JWTAuthorized,
    client: &DBClient,
    friend_id: String,
) -> Result<(), Custom<String>> {
    if &auth.0 == &friend_id {
        return Err(Custom(
            Status::BadRequest,
            "you cant add yourself".to_string(),
        ));
    }

    let exists_res = does_user_exist(client, friend_id.clone()).await;
    match exists_res {
        Ok(exists) => {
            if !exists {
                return Err(Custom(
                    Status::BadRequest,
                    "no user exists with that id".to_string(),
                ));
            }
        }
        Err(_) => {
            return Err(Custom(
                Status::BadRequest,
                "unable to get friend".to_string(),
            ));
        }
    }

    let existing_requests_res = get_outgoing_friend_requests(client, auth.0.clone()).await;
    match existing_requests_res {
        Ok(existing_requests) => {
            if existing_requests
                .into_iter()
                .any(|er| -> bool { &er.friend_id == &friend_id })
            {
                return Err(Custom(
                    Status::BadRequest,
                    "request already sent".to_string(),
                ));
            }

            let friends_res = get_current_friends(client, auth.0.clone()).await;

            match friends_res {
                Ok(friends) => {
                    if friends
                        .into_iter()
                        .any(|f| -> bool { &f.friend_id == &friend_id })
                    {
                        return Err(Custom(Status::BadRequest, "already friends".to_string()));
                    }

                    let create_res =
                        create_friend_request(client, auth.0.clone().as_str(), &friend_id).await;
                    match create_res {
                        Ok(_) => return Ok(()),
                        Err(_) => {
                            return Err(Custom(
                                Status::InternalServerError,
                                "error creating friend request".to_string(),
                            ))
                        }
                    }
                }
                Err(_) => {
                    return Err(Custom(
                        Status::InternalServerError,
                        "unable to get friends".to_string(),
                    ))
                }
            }
        }
        Err(_) => {
            return Err(Custom(
                Status::InternalServerError,
                "unable to fetch existing requests".to_string(),
            ))
        }
    }
}

#[post("/accept_request?<request_id>")]
pub async fn accept_request(
    auth: JWTAuthorized,
    client: &DBClient,
    request_id: String,
) -> Result<(), Custom<String>> {
    let friend_requests_res = get_incoming_friend_requests(client, auth.0.clone()).await;

    match friend_requests_res {
        Ok(friend_requests) => {
            let request_opt = friend_requests.into_iter().find(|fr| -> bool {
                return fr.id == request_id;
            });

            match request_opt {
                Some(request) => {
                    let accept_res =
                        accept_friend_request(&client, &auth.0.clone(), &request.user_id).await;

                    match accept_res {
                        Ok(_) => Ok(()),
                        Err(_) => Err(Custom(
                            Status::BadRequest,
                            "failed accepting friend request".to_string(),
                        )),
                    }
                }
                None => {
                    return Err(Custom(
                        Status::BadRequest,
                        "friend request does not exist".to_string(),
                    ));
                }
            }
        }
        Err(_) => {
            return Err(Custom(
                Status::InternalServerError,
                "could not fetch incoming friend requests".to_string(),
            ))
        }
    }
}

#[post("/cancel_request?<request_id>")]
pub async fn cancel_request(
    auth: JWTAuthorized,
    client: &DBClient,
    request_id: String,
) -> Result<(), Custom<String>> {
    let friend_requests_res = get_outgoing_friend_requests(client, auth.0.clone()).await;

    match friend_requests_res {
        Ok(friend_requests) => {
            let friend_request_exists = friend_requests.into_iter().any(|fr| -> bool {
                return fr.id == request_id;
            });

            if friend_request_exists {
                let cancel_res = cancel_friend_request(&client, &request_id, &auth.0.clone()).await;

                match cancel_res {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Custom(
                        Status::BadRequest,
                        "failed cancelling friend request".to_string(),
                    )),
                }
            } else {
                return Err(Custom(
                    Status::BadRequest,
                    "friend request does not exist".to_string(),
                ));
            }
        }
        Err(_) => {
            return Err(Custom(
                Status::InternalServerError,
                "could not fetch incoming friend requests".to_string(),
            ))
        }
    }
}

#[post("/ignore_request?<request_id>")]
pub async fn ignore_request(
    auth: JWTAuthorized,
    client: &DBClient,
    request_id: String,
) -> Result<(), Custom<String>> {
    let friend_requests_res = get_incoming_friend_requests(client, auth.0.clone()).await;

    match friend_requests_res {
        Ok(friend_requests) => {
            let friend_request_exists = friend_requests.into_iter().any(|fr| -> bool {
                return fr.id == request_id;
            });

            if friend_request_exists {
                let ignore_res = ignore_friend_request(&client, &request_id, &auth.0.clone()).await;

                match ignore_res {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Custom(
                        Status::BadRequest,
                        "failed ignoring friend request".to_string(),
                    )),
                }
            } else {
                return Err(Custom(
                    Status::BadRequest,
                    "friend request does not exist".to_string(),
                ));
            }
        }
        Err(_) => {
            return Err(Custom(
                Status::InternalServerError,
                "could not fetch incoming friend requests".to_string(),
            ))
        }
    }
}

#[post("/decline_request?<request_id>")]
pub async fn decline_request(
    auth: JWTAuthorized,
    client: &DBClient,
    request_id: String,
) -> Result<(), Custom<String>> {
    let friend_requests_res = get_incoming_friend_requests(client, auth.0.clone()).await;

    match friend_requests_res {
        Ok(friend_requests) => {
            let friend_request_exists = friend_requests.into_iter().any(|fr| -> bool {
                return fr.id == request_id;
            });

            if friend_request_exists {
                let ignore_res =
                    decline_friend_request(&client, &request_id, &auth.0.clone()).await;

                match ignore_res {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Custom(
                        Status::BadRequest,
                        "failed declining friend request".to_string(),
                    )),
                }
            } else {
                return Err(Custom(
                    Status::BadRequest,
                    "friend request does not exist".to_string(),
                ));
            }
        }
        Err(_) => {
            return Err(Custom(
                Status::InternalServerError,
                "could not fetch incoming friend requests".to_string(),
            ))
        }
    }
}

#[post("/remove?<friend_id>")]
pub async fn remove_friend(
    auth: JWTAuthorized,
    client: &DBClient,
    friend_id: String,
) -> Result<(), Custom<String>> {
    let friends_res = get_current_friends(client, auth.0.clone()).await;

    match friends_res {
        Ok(friends) => {
            let friend_exists = friends.into_iter().any(|fr| -> bool {
                return fr.friend_id == friend_id;
            });

            if friend_exists {
                let remove_res = remove_current_friend(&client, &auth.0.clone(), &friend_id).await;

                match remove_res {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Custom(
                        Status::BadRequest,
                        "failed removing friend".to_string(),
                    )),
                }
            } else {
                return Err(Custom(
                    Status::BadRequest,
                    "friend request does not exist".to_string(),
                ));
            }
        }
        Err(_) => {
            return Err(Custom(
                Status::InternalServerError,
                "could not fetch incoming friend requests".to_string(),
            ))
        }
    }
}
