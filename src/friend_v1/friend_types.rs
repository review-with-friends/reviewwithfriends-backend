use crate::db::{Friend, FriendRequest};
use serde::Serialize;

/// All in one return type for fetching all friend information.
#[derive(Serialize)]
pub struct FullFriendsPub {
    pub friends: Vec<FriendPub>,
    pub incoming_requests: Vec<FriendRequestPub>,
    pub outgoing_requests: Vec<FriendRequestPub>,
    pub ignored_requests: Vec<FriendRequestPub>,
}

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
pub struct FriendPub {
    pub id: String,
    pub created: i64,
    pub user_id: String,
    pub friend_id: String,
}

impl From<Friend> for FriendPub {
    fn from(friend: Friend) -> FriendPub {
        FriendPub {
            id: friend.id,
            created: friend.created.timestamp_millis(),
            user_id: friend.user_id,
            friend_id: friend.friend_id,
        }
    }
}

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
pub struct FriendRequestPub {
    pub id: String,
    pub created: i64,
    pub user_id: String,
    pub friend_id: String,
}

impl From<FriendRequest> for FriendRequestPub {
    fn from(friend_request: FriendRequest) -> FriendRequestPub {
        FriendRequestPub {
            id: friend_request.id,
            created: friend_request.created.timestamp_millis(),
            user_id: friend_request.user_id,
            friend_id: friend_request.friend_id,
        }
    }
}
