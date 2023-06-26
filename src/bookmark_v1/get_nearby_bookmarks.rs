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
pub struct NearbyBookmarksRequest {
    latitude: f64,
    longitude: f64,
    user_id: String,
    page: u32,
}

/// Gets all the replies for a given review.
#[get("/all_nearby_bookmarks")]
pub async fn get_nearby_all_bookmarks(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    nearby_bookmarks_request: Query<NearbyBookmarksRequest>,
) -> Result<impl Responder> {
    let bookmark_res = db::get_nearby_bookmarks(
        &pool,
        &authenticated_user.0,
        &nearby_bookmarks_request.user_id,
        nearby_bookmarks_request.page,
        nearby_bookmarks_request.latitude,
        nearby_bookmarks_request.longitude,
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
