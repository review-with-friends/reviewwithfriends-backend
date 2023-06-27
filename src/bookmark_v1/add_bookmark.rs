use crate::{
    authorization::AuthenticatedUser,
    db::{self, Bookmark},
    tracing::add_error_span,
};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Query, ReqData},
    HttpResponse, Responder, Result,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::MySqlPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AddBookmarkRequest {
    pub location_name: String,
    pub category: String,
    pub latitude: f64,
    pub longitude: f64,
}

/// Gets all the replies for a given review.
#[post("")]
pub async fn add_bookmark(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    add_bookmark_request: Query<AddBookmarkRequest>,
) -> Result<impl Responder> {
    let does_bookmark_exist = db::does_bookmark_exist(
        &pool,
        &authenticated_user.0,
        &add_bookmark_request.location_name,
        add_bookmark_request.latitude,
        add_bookmark_request.longitude,
    )
    .await;

    if let Ok(bookmark_exists) = does_bookmark_exist {
        if bookmark_exists {
            return Ok(HttpResponse::Ok().finish());
        }
    } else {
        return Err(ErrorInternalServerError(
            "unable to get existing bookmarks".to_string(),
        ));
    }

    let bookmark = Bookmark {
        id: Uuid::new_v4().to_string(),
        user_id: authenticated_user.0.to_string(),
        created: Utc::now().naive_utc(),
        category: add_bookmark_request.category.clone(),
        location_name: add_bookmark_request.location_name.clone(),
        latitude: add_bookmark_request.latitude,
        longitude: add_bookmark_request.longitude,
    };

    let add_bookmark_res = db::create_bookmark(&pool, &bookmark).await;

    match add_bookmark_res {
        Ok(_) => {
            return Ok(HttpResponse::Ok().finish());
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError(
                "unable to create bookmark".to_string(),
            ));
        }
    }
}
