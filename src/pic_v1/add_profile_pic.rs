use crate::{
    authorization::AuthenticatedUser,
    db::{create_pic, get_user, update_user_pic_id},
};
use actix_web::{
    post,
    web::{Bytes, Data, ReqData},
    HttpResponse, Result,
};
use images::{ByteStream, PutObjectRequest, S3Client, S3};
use sqlx::MySqlPool;
use validation::validate_profile_pic;

use super::shared_utils::best_effort_delete_pic;

/// Allows users to update their profile pic.
#[post("/profile_pic")]
pub async fn add_profile_pic(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    s3_client: Data<S3Client>,
    pic_bytes: Bytes,
) -> Result<HttpResponse> {
    let validation_result = validate_profile_pic(&pic_bytes);

    let width: u16;
    let height: u16;

    match validation_result {
        Ok(size) => {
            width = size.0;
            height = size.1;
        }
        Err(err) => {
            return Ok(HttpResponse::BadRequest().body(err));
        }
    }

    let previous_pic_id: String;
    let user_res = get_user(&pool, &authenticated_user.0).await;

    match user_res {
        Ok(user_opt) => {
            if let Some(user) = user_opt {
                previous_pic_id = user.pic_id;
            } else {
                return Ok(HttpResponse::NotFound().body("could not find user"));
            }
        }
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().body("unable to get user"));
        }
    }

    let pic_res = create_pic(&pool, None, width, height).await;

    match pic_res {
        Ok(pic) => {
            // We'll always put profile pics here into DO.
            // This saves a join on querying users.
            if let Err(_) = s3_client
                .put_object(PutObjectRequest {
                    body: Some(ByteStream::from(<Vec<u8>>::from(pic_bytes))),
                    bucket: "bout".to_string(),
                    key: pic.id.clone(),
                    acl: Some("public-read".to_string()),
                    ..Default::default()
                })
                .await
            {
                return Ok(HttpResponse::InternalServerError().body("unable to store profile pic"));
            }

            if let Err(_) = update_user_pic_id(&pool, &pic.id, &authenticated_user.0).await {
                return Ok(HttpResponse::InternalServerError().body("unable to save profile pic"));
            }

            best_effort_delete_pic(&s3_client, &pool, &previous_pic_id).await; // best effort - we can clean up stored images later

            return Ok(HttpResponse::Ok().finish());
        }
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().body("unable to create profile pic"));
        }
    }
}
