use crate::{authorization::AuthenticatedUser, db};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Json, Query, ReqData},
    Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

use super::bookmark_types::BookmarkPub;

#[derive(Deserialize)]
pub struct GetUserBookmarksRequest {
    user_id: String,
}

/// Gets all the replies for a given review.
#[get("/all_by_user")]
pub async fn get_all_bookmarks(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    get_user_bookmarks_request: Query<GetUserBookmarksRequest>,
) -> Result<impl Responder> {
    let bookmark_res = db::get_all_bookmarks(
        &pool,
        &authenticated_user.0,
        &get_user_bookmarks_request.user_id,
    )
    .await;

    match bookmark_res {
        Ok(bookmarks) => {
            let bookmark_pubs: Vec<BookmarkPub> = bookmarks
                .into_iter()
                .map(|f| -> BookmarkPub { f.into() })
                .collect();
            return Ok(Json(bookmark_pubs));
        }
        Err(_) => {
            return Err(ErrorInternalServerError(
                "unable to get bookmarks".to_string(),
            ))
        }
    }
}
