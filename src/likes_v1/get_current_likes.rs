use crate::{authorization::AuthenticatedUser, db::get_liked_reviews, review_v1::ReviewPub};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Json, Query, ReqData},
    Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct GetPagedCurrentLikes {
    page: u32,
}

/// Gets all the reviews the user has liked.
#[get("/current")]
pub async fn get_current_likes(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    get_paged_current_likes: Query<GetPagedCurrentLikes>,
) -> Result<impl Responder> {
    let reviews_res =
        get_liked_reviews(&pool, &authenticated_user.0, get_paged_current_likes.page).await;

    match reviews_res {
        Ok(reviews) => {
            let reviews_pub: Vec<ReviewPub> = reviews
                .into_iter()
                .map(|f| -> ReviewPub { f.into() })
                .collect();
            return Ok(Json(reviews_pub));
        }
        Err(_) => return Err(ErrorInternalServerError("unable to get likes".to_string())),
    }
}
