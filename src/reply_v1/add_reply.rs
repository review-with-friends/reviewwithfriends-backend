use crate::{
    authorization::AuthenticatedUser,
    db::{create_notification, create_reply, get_reply, get_review, get_user, Review},
    notifications_v1::{
        enqueue_notification, ActionType, NotificationQueue, NotificationQueueItem,
        NotificationType,
    },
    tracing::add_error_span,
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::{Data, Json, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;
use std::sync::Mutex;
use validation::validate_reply_text;

#[derive(Deserialize)]
pub struct AddReplyRequest {
    text: String,
    review_id: String,
    reply_to_id: Option<String>,
}

/// Allows users to add a reply linked to a review.
#[post("")]
pub async fn add_reply(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    apn_queue: Data<Mutex<NotificationQueue>>,
    add_reply_request: Json<AddReplyRequest>,
) -> Result<impl Responder> {
    if let Err(err) = validate_reply_text(&add_reply_request.text) {
        return Err(ErrorBadRequest(err));
    }

    let review_res = get_review(&pool, &authenticated_user.0, &add_reply_request.review_id).await;

    let review: Review;

    match review_res {
        Ok(review_opt) => {
            if let Some(review_tmp) = review_opt {
                review = review_tmp;
            } else {
                return Err(ErrorBadRequest("unable to find review".to_string()));
            }
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError("unable to get review".to_string()));
        }
    }

    let reply_res = create_reply(
        &pool,
        &authenticated_user.0,
        &add_reply_request.review_id,
        &add_reply_request.text,
        add_reply_request.reply_to_id.as_ref(),
    )
    .await;

    match reply_res {
        Ok(_) => {
            enqueue_reply_notification_to_author(&pool, &apn_queue, &review, &authenticated_user.0)
                .await;

            if let Some(reply_to_id) = &add_reply_request.reply_to_id {
                enqueue_reply_notification_to_reply_id(
                    &pool,
                    &apn_queue,
                    &review,
                    &authenticated_user.0,
                    reply_to_id,
                )
                .await;
            }

            return Ok(HttpResponse::Ok().finish());
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError("unable create reply".to_string()));
        }
    }
}

/// Generally a best-effort async function to get notifications out.
async fn enqueue_reply_notification_to_author(
    pool: &Data<MySqlPool>,
    apn_queue: &Data<Mutex<NotificationQueue>>,
    review: &Review,
    authenticated_user_id: &str,
) {
    let _ = create_notification(
        &pool,
        authenticated_user_id,
        &review.user_id,
        &review.id,
        ActionType::Reply.into(),
    )
    .await;

    let user_res = get_user(&pool, &review.user_id).await;

    // Best effort sending the notification through apple sevices.
    match user_res {
        Ok(user_opt) => {
            if let Some(user) = user_opt {
                let calling_user_res = get_user(&pool, authenticated_user_id).await;

                match calling_user_res {
                    Ok(calling_user_opt) => {
                        if let Some(calling_user) = calling_user_opt {
                            if calling_user.id != user.id {
                                enqueue_notification(
                                    NotificationQueueItem {
                                        user_id: user.id.to_string(),
                                        notification_value: Some(review.id.to_string()),
                                        message: format!(
                                            "{} replied to your review!",
                                            calling_user.display_name
                                        ),
                                        notification_type: NotificationType::Reply,
                                    },
                                    &apn_queue,
                                );
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_) => {}
    }
}

/// Generally a best-effort async function to get notifications out.
async fn enqueue_reply_notification_to_reply_id(
    pool: &Data<MySqlPool>,
    apn_queue: &Data<Mutex<NotificationQueue>>,
    review: &Review,
    authenticated_user_id: &str,
    reply_to_id: &str,
) {
    let reply_res = get_reply(pool, &review.id, reply_to_id).await;

    match reply_res {
        Ok(reply_opt) => {
            // We found valid reply this reply is replying to.
            if let Some(reply) = reply_opt {
                let _ = create_notification(
                    &pool,
                    authenticated_user_id,
                    &reply.user_id,
                    &review.id,
                    ActionType::ReplyTo.into(),
                )
                .await;

                // get the user who owns the reply we are replying to
                let user_res = get_user(&pool, &reply.user_id).await;

                // Best effort sending the notification through apple sevices.
                match user_res {
                    Ok(user_opt) => {
                        // we actually have the user now, big chill
                        // user is the person who should be getting the notification
                        if let Some(user) = user_opt {
                            let calling_user_res = get_user(&pool, authenticated_user_id).await;

                            match calling_user_res {
                                Ok(calling_user_opt) => {
                                    // we have the calling user now
                                    // this is the person who's name will show up within the notification text
                                    if let Some(calling_user) = calling_user_opt {
                                        // ensure we don't notify ourselves.
                                        // this should be checked for high tbqh but w/e.
                                        if calling_user.id != user.id {
                                            // send notification referencing a specific review, to a given user, with generated text.
                                            enqueue_notification(
                                                NotificationQueueItem {
                                                    user_id: user.id.to_string(),
                                                    notification_value: Some(review.id.to_string()),
                                                    message: format!(
                                                        "{} replied to you!",
                                                        calling_user.display_name
                                                    ),
                                                    notification_type: NotificationType::Reply,
                                                },
                                                &apn_queue,
                                            );
                                        }
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                    }
                    Err(_) => {}
                }
            } else {
                // this is fine, another race condition possibility.
            }
        }
        Err(_) => {
            // this is fine, this means the reply was deleted before this was complete
            // or bad data was passed
        }
    }
}
