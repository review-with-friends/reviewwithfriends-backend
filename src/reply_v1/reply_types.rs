use crate::db::Reply;
use serde::Serialize;

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
pub struct ReplyPub {
    pub id: String,
    pub created: i64,
    pub user_id: String,
    pub review_id: String,
    pub text: String,
    pub reply_to_id: Option<String>,
}

impl From<Reply> for ReplyPub {
    fn from(reply: Reply) -> ReplyPub {
        ReplyPub {
            id: reply.id,
            created: reply.created.timestamp_millis(),
            user_id: reply.user_id,
            review_id: reply.review_id,
            text: reply.text,
            reply_to_id: reply.reply_to_id,
        }
    }
}
