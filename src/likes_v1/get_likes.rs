use crate::{
    authorization::AuthenticatedUser,
    db::{get_all_likes, get_review},
};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Json, Query, ReqData},
    Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

use super::like_types::LikePub;

#[derive(Deserialize)]
pub struct GetUserByIdRequest {
    review_id: String,
}

/// Gets all the users who have liked a post.
#[get("")]
pub async fn get_likes(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    get_user_request: Query<GetUserByIdRequest>,
) -> Result<impl Responder> {
    if let Err(_) = get_review(&pool, &authenticated_user.0, &get_user_request.review_id).await {
        return Err(ErrorInternalServerError(
            "unable to find review".to_string(),
        ));
    }

    let likes_res = get_all_likes(&pool, &get_user_request.review_id).await;

    match likes_res {
        Ok(likes) => {
            let likes_pub: Vec<LikePub> =
                likes.into_iter().map(|f| -> LikePub { f.into() }).collect();
            return Ok(Json(likes_pub));
        }
        Err(_) => return Err(ErrorInternalServerError("unable to get likes".to_string())),
    }
}
