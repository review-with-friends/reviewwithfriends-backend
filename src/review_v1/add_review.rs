use std::sync::Mutex;

use crate::{
    authorization::AuthenticatedUser,
    db::{
        create_pic, create_review, get_current_friends, get_user, remove_review_and_children,
        remove_review_pic_id, Review,
    },
    notifications_v1::{
        enqueue_notification, NotificationQueue, NotificationQueueItem, NotificationType,
    },
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::{Data, Json, ReqData},
    HttpResponse, Responder, Result,
};
use base64::{engine::general_purpose, Engine};
use chrono::{NaiveDateTime, Utc};
use images::{ByteStream, PutObjectRequest, S3Client, S3};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;
use validation::{
    validate_latitude, validate_location_name, validate_longitude, validate_review_category,
    validate_review_pic_b64, validate_review_text, validate_stars,
};

use super::review_types::ReviewPub;

#[derive(Deserialize, Serialize)]
pub struct AddReviewRequest {
    pub text: String,
    pub stars: u8,
    pub category: String,
    pub location_name: String,
    pub pic: String,
    pub pic_p: Option<String>,
    pub pic_q: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub is_custom: bool,
    pub post_date: Option<i64>,
}

/// Allows the user to create a review for a specific place.
#[post("/")]
pub async fn add_review(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    s3_client: Data<S3Client>,
    apn_queue: Data<Mutex<NotificationQueue>>,
    add_review_request: Json<AddReviewRequest>,
) -> Result<impl Responder> {
    if let Err(err) = validate_review_text(&add_review_request.text) {
        return Err(ErrorBadRequest(err.to_string()));
    }

    if let Err(err) = validate_longitude(add_review_request.longitude) {
        return Err(ErrorBadRequest(err.to_string()));
    }

    if let Err(err) = validate_latitude(add_review_request.latitude) {
        return Err(ErrorBadRequest(err.to_string()));
    }

    if let Err(err) = validate_location_name(&add_review_request.location_name) {
        return Err(ErrorBadRequest(err.to_string()));
    }

    if let Err(err) = validate_stars(add_review_request.stars) {
        return Err(ErrorBadRequest(err.to_string()));
    }

    if let Err(err) = validate_review_category(&add_review_request.category) {
        return Err(ErrorBadRequest(err.to_string()));
    }

    let validation_result = validate_pics(&add_review_request);

    let pics: Vec<PicToUpload>;

    match validation_result {
        Ok(pics_tmp) => {
            pics = pics_tmp;
        }
        Err(err) => {
            return Ok(HttpResponse::BadRequest().body(err));
        }
    }

    let review = map_review_to_db(&add_review_request, &authenticated_user.0);

    let create_res = create_review(&pool, &review).await;

    match create_res {
        Ok(_) => {
            let pic_upload_res = upload_and_store_pics(s3_client, &pool, &review, pics).await;

            match pic_upload_res {
                Ok(_) => {
                    let friends_res = get_current_friends(&pool, &authenticated_user.0).await;

                    if let Ok(friends) = friends_res {
                        let calling_user_res = get_user(&pool, &authenticated_user.0).await;
                        if let Ok(calling_user_opt) = calling_user_res {
                            if let Some(calling_user) = calling_user_opt {
                                for friend in friends {
                                    if calling_user.id != friend.friend_id {
                                        enqueue_notification(
                                            NotificationQueueItem {
                                                user_id: friend.friend_id.to_string(),
                                                notification_value: Some(review.id.to_string()),
                                                message: format!(
                                                    "{} posted a new review!",
                                                    calling_user.display_name
                                                ),
                                                notification_type: NotificationType::Post,
                                            },
                                            &apn_queue,
                                        );
                                    }
                                }
                            }
                        }
                    }

                    return Ok(HttpResponse::Ok().json(ReviewPub::from(review)));
                }
                Err(err) => return Err(ErrorInternalServerError(err)),
            }
        }
        Err(_) => {
            return Err(ErrorInternalServerError("failed to create review"));
        }
    }
}

fn validate_pics(add_review_request: &AddReviewRequest) -> Result<Vec<PicToUpload>, String> {
    let mut output: Vec<PicToUpload> = vec![];

    let validation_result = validate_review_pic_b64(&add_review_request.pic);

    match validation_result {
        Ok(size) => output.push(PicToUpload {
            data: &add_review_request.pic,
            width: size.0,
            height: size.1,
        }),
        Err(err) => {
            return Err(err);
        }
    }

    if let Some(pic_p) = &add_review_request.pic_p {
        let validation_result = validate_review_pic_b64(pic_p);

        match validation_result {
            Ok(size) => output.push(PicToUpload {
                data: &pic_p,
                width: size.0,
                height: size.1,
            }),
            Err(err) => {
                return Err(err);
            }
        }
    }

    if let Some(pic_q) = &add_review_request.pic_q {
        let validation_result = validate_review_pic_b64(pic_q);

        match validation_result {
            Ok(size) => output.push(PicToUpload {
                data: &pic_q,
                width: size.0,
                height: size.1,
            }),
            Err(err) => {
                return Err(err);
            }
        }
    }

    return Ok(output);
}

struct PicToUpload<'a> {
    data: &'a str,
    width: u16,
    height: u16,
}

async fn upload_and_store_pics(
    s3_client: Data<S3Client>,
    pool: &MySqlPool,
    review: &Review,
    pics: Vec<PicToUpload<'_>>,
) -> Result<(), String> {
    for pic_data in pics {
        let pic_res = create_pic(
            &pool,
            Some(review.id.clone()),
            pic_data.width,
            pic_data.height,
        )
        .await;

        match pic_res {
            Ok(pic) => {
                if let Ok(bytes) = general_purpose::STANDARD.decode(pic_data.data) {
                    if let Err(_) = s3_client
                        .put_object(PutObjectRequest {
                            body: Some(ByteStream::from(bytes)),
                            bucket: "bout".to_string(),
                            key: pic.id.clone(),
                            acl: Some("public-read".to_string()),
                            ..Default::default()
                        })
                        .await
                    {
                        let _ = remove_review_pic_id(&pool, &pic.id, &review.id).await;
                        let _ = remove_review_and_children(&pool, &review.id).await;

                        return Err("unable to store review pic".to_string());
                    }
                } else {
                    let _ = remove_review_pic_id(&pool, &pic.id, &review.id).await;
                    let _ = remove_review_and_children(&pool, &review.id).await;

                    return Err("failed to read pic buffer".to_string());
                }
            }
            Err(_) => {
                return Err("unable to create review pic".to_string());
            }
        }
    }

    return Ok(());
}

fn map_review_to_db(request: &AddReviewRequest, user_id: &str) -> Review {
    let post_date: NaiveDateTime;

    if let Some(user_post_date) = request.post_date {
        if let Some(ndt) = NaiveDateTime::from_timestamp_millis(user_post_date) {
            post_date = ndt;
        } else {
            post_date = Utc::now().naive_utc();
        }
    } else {
        post_date = Utc::now().naive_utc();
    }

    Review {
        id: Uuid::new_v4().to_string(),
        user_id: user_id.to_string(),
        created: post_date,
        pic_id: None,
        category: request.category.clone(),
        text: request.text.clone(),
        stars: request.stars,
        location_name: request.location_name.clone(),
        latitude: request.latitude,
        longitude: request.longitude,
        is_custom: request.is_custom,
    }
}
