use crate::db::Notification;
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
}

impl From<Notification> for NotificationPub {
    fn from(notification: Notification) -> NotificationPub {
        NotificationPub {
            id: notification.id,
            created: notification.created.timestamp_millis(),
            review_user_id: notification.review_user_id,
            user_id: notification.user_id,
            review_id: notification.review_id,
            action_type: notification.action_type,
        }
    }
}

/// ActionType for associated notification records.
pub enum ActionType {
    Unknown = 0,
    Like = 1,
    Reply = 2,
}

impl From<ActionType> for u8 {
    fn from(action_type: ActionType) -> u8 {
        match action_type {
            ActionType::Unknown => 0,
            ActionType::Like => 1,
            ActionType::Reply => 2,
        }
    }
}
