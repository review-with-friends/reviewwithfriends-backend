use std::sync::Mutex;

use crate::{
    authorization::AuthenticatedUser,
    db::{create_like, create_notification, get_review, get_user, is_already_liked, Review},
    notifications_v1::{
        enqueue_notification, ActionType, NotificationQueue, NotificationQueueItem,
        NotificationType,
    },
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    post,
    web::{Data, Query, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct LikeReviewRequest {
    pub review_id: String,
}

/// Allows users to like a review.
#[post("")]
pub async fn like_review(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    apn_queue: Data<Mutex<NotificationQueue>>,
    like_review_request: Query<LikeReviewRequest>,
) -> Result<impl Responder> {
    let review_res = get_review(&pool, &authenticated_user.0, &like_review_request.review_id).await;

    let review: Review;
    match review_res {
        Ok(review_opt) => {
            if let Some(review_tmp) = review_opt {
                review = review_tmp
            } else {
                return Err(ErrorNotFound("could not find review"));
            }
        }
        Err(_) => return Err(ErrorInternalServerError("failed to get review")),
    }

    let already_created_res =
        is_already_liked(&pool, &authenticated_user.0, &like_review_request.review_id).await;

    match already_created_res {
        Ok(is_liked) => {
            if is_liked {
                return Ok(HttpResponse::Ok().finish());
            }
        }
        Err(_) => {
            return Err(ErrorInternalServerError("failed to fetch existing likes"));
        }
    }

    let create_res =
        create_like(&pool, &authenticated_user.0, &like_review_request.review_id).await;

    match create_res {
        Ok(_) => {
            // Creating the notification is best effort. We may look into not awaiting this;
            // though unsure of how the tokio runtime closes out the webrequest.
            let _ = create_notification(
                &pool,
                &authenticated_user.0,
                &review.user_id,
                &review.id,
                ActionType::Like.into(),
            )
            .await;

            let user_res = get_user(&pool, &review.user_id).await;

            // Best effort sending the notification through apple sevices.
            match user_res {
                Ok(user_opt) => {
                    if let Some(user) = user_opt {
                        let calling_user_res = get_user(&pool, &authenticated_user.0).await;

                        match calling_user_res {
                            Ok(calling_user_opt) => {
                                if let Some(calling_user) = calling_user_opt {
                                    if calling_user.id != user.id {
                                        enqueue_notification(
                                            NotificationQueueItem {
                                                user_id: user.id.to_string(),
                                                notification_value: Some(review.id.to_string()),
                                                message: format!(
                                                    "{} added your review to their favorites!",
                                                    calling_user.display_name
                                                ),
                                                notification_type: NotificationType::Favorite,
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

            return Ok(HttpResponse::Ok().finish());
        }
        Err(_) => {
            return Err(ErrorInternalServerError("failed to like review"));
        }
    }
}
