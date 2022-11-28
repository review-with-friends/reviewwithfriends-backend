use crate::{
    authorization::AuthenticatedUser,
    db::{get_pic, get_review, Pic},
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
pub struct ReviewPicRequest {
    review_id: String,
}

#[get("/review_pic")]
pub async fn get_review_pic(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    s3_client: Data<S3Client>,
    review_pic_request: Query<ReviewPicRequest>,
) -> Result<HttpResponse> {
    let pic_id: String;
    if let Ok(review_opt) =
        get_review(&pool, &authenticated_user.0, &review_pic_request.review_id).await
    {
        if let Some(review) = review_opt {
            if let Some(_pic_id) = review.pic_id {
                pic_id = _pic_id;
            } else {
                return Ok(HttpResponse::NotFound().body("review has no pic"));
            }
        } else {
            return Ok(HttpResponse::NotFound().body("could not find review"));
        }
    } else {
        return Ok(HttpResponse::NotFound().body("failed to get review"));
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
        return Ok(HttpResponse::InternalServerError().body("failed to fetch pic from storage"));
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
