use crate::db::get_user;

use super::APNClient;
use actix_web::web::Data;
use opentelemetry::global;
use opentelemetry::trace::{Span, Status, Tracer};
use sqlx::MySqlPool;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::{sync::Mutex, time::Duration};
use tokio::{task, time};

pub struct NotificationQueue(VecDeque<NotificationQueueItem>);

impl NotificationQueue {
    /// Initialize a new queue
    pub fn new() -> Self {
        NotificationQueue(vec![].into())
    }
}

impl Deref for NotificationQueue {
    type Target = VecDeque<NotificationQueueItem>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NotificationQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Used to contain the notification information
pub struct NotificationQueueItem {
    /// User Id who will receieve the notification
    pub user_id: String,

    /// Review id associated with the notification
    pub review_id: Option<String>,

    /// User friendly message for the notification
    pub message: String,

    /// The type of notification being sent
    pub notification_type: NotificationType,
}

/// Type of notificaton being sent. This changes the underlying
/// deep linking for said notification
pub enum NotificationType {
    /// When a user favorites (likes) a post
    Favorite,
    /// When a user replies to a post
    Reply,
    /// When a user sends a new friend request
    Add,
    /// When a user posts a new review
    Post,
}

/// Starts a background task to process the queue
pub fn start_notification_worker(
    queue: Data<Mutex<NotificationQueue>>,
    client: Data<APNClient>,
    pool: Data<MySqlPool>,
) {
    task::spawn(async move {
        loop {
            let mut dequeued_item: Option<NotificationQueueItem> = None;

            if let Ok(mut queue) = queue.lock() {
                dequeued_item = queue.pop_front();
            }

            if let Some(item) = dequeued_item {
                let tracer = global::tracer("Apple Push Notification");
                let mut span = tracer.start("Notification Sent");

                if let Ok(user_opt) = get_user(&pool, &item.user_id).await {
                    if let Some(user) = user_opt {
                        if let Some(device_token) = user.device_token {
                            let res = client.send_notification(&device_token, &item.message).await;
                            if let Err(err) = res {
                                span.set_status(Status::error(err.clone()));
                            }
                        }
                    }
                }

                span.end();
            }

            time::sleep(Duration::from_millis(100)).await;
        }
    });
}

/// Add a NotificationQueueItem to the queue
pub fn enqueue_notification(item: NotificationQueueItem, queue: &Data<Mutex<NotificationQueue>>) {
    if let Ok(mut queue) = queue.lock() {
        queue.push_back(item);
    }
}
