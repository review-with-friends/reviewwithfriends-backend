use crate::db::ExpandedNotification;
use serde::Serialize;

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
pub struct NotificationPub {
    pub id: String,
    pub created: i64,
    pub review_user_id: String,
    pub user_id: String,
    pub review_id: String,
    pub action_type: u8,
    pub review_location: String,
}

impl From<ExpandedNotification> for NotificationPub {
    fn from(notification: ExpandedNotification) -> NotificationPub {
        NotificationPub {
            id: notification.id,
            created: notification.created.timestamp_millis(),
            review_user_id: notification.review_user_id,
            user_id: notification.user_id,
            review_id: notification.review_id,
            action_type: notification.action_type,
            review_location: notification.review_location,
        }
    }
}

/// ActionType for associated notification records.
pub enum ActionType {
    /// Unknown/default yolo.
    Unknown = 0,

    /// When someone likes your review.
    Like = 1,

    /// When someone comments to your review.
    Reply = 2,

    /// When someone replies to your reply.
    ReplyTo = 3,
}

impl From<ActionType> for u8 {
    fn from(action_type: ActionType) -> u8 {
        match action_type {
            ActionType::Unknown => 0,
            ActionType::Like => 1,
            ActionType::Reply => 2,
            ActionType::ReplyTo => 3,
        }
    }
}
