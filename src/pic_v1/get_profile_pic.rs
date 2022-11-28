use crate::{
    authorization::AuthenticatedUser,
    db::{get_pic, get_user, Pic},
};
use actix_web::{
    get,
    http::header::ContentType,
    web::{Data, Query, ReqData},
    HttpResponse, Result,
};
use images::{GetObjectOutput, GetObjectRequest, S3Client, S3};
use serde::Deserialize;
use sqlx::MySqlPool;
use tokio::io::AsyncReadExt;

#[derive(Deserialize)]
pub struct ProfilePicRequest {
    user_id: String,
}

/// Fetches a profile pic set for a user.
#[get("/profile_pic")]
pub async fn get_profile_pic(
    _authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    s3_client: Data<S3Client>,
    avatar_request: Query<ProfilePicRequest>,
) -> Result<HttpResponse> {
    let pic_id: String;
    if let Ok(user_opt) = get_user(&pool, &avatar_request.user_id).await {
        if let Some(user) = user_opt {
            pic_id = user.pic_id;
        } else {
            return Ok(HttpResponse::NotFound().body("could not find user"));
        }
    } else {
        return Ok(HttpResponse::InternalServerError().body("failed to get user"));
    }

    let pic: Pic;
    if let Ok(pic_opt) = get_pic(&pool, &pic_id).await {
        if let Some(pic_tmp) = pic_opt {
            pic = pic_tmp;
        } else {
            return Ok(HttpResponse::NotFound().body("pic not found"));
        }
    } else {
        return Ok(HttpResponse::InternalServerError().body("failed to fetch pic"));
    }

    let pic_obj: GetObjectOutput;
    if let Ok(pic_obj_) = s3_client
        .get_object(GetObjectRequest {
            bucket: "bout".to_string(),
            key: pic.id,
            ..Default::default()
        })
        .await
    {
        pic_obj = pic_obj_;
    } else {
        return Ok(HttpResponse::InternalServerError().body("failed to fetch pic from db"));
    }

    let mut buf: Vec<u8> = Vec::new();

    if let Err(_) = pic_obj
        .body
        .unwrap()
        .into_async_read()
        .read_to_end(&mut buf)
        .await
    {
        return Ok(HttpResponse::InternalServerError().body("failed to fetch pic from storage"));
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::jpeg())
        .body(buf))
}
