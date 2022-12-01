use crate::db::Like;
use serde::Serialize;

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
pub struct LikePub {
    pub id: String,
    pub created: i64,
    pub user_id: String,
    pub review_id: String,
}

impl From<Like> for LikePub {
    fn from(like: Like) -> LikePub {
        LikePub {
            id: like.id,
            created: like.created.timestamp_millis(),
            user_id: like.user_id,
            review_id: like.review_id,
        }
    }
}
