use crate::{
    authorization::AuthenticatedUser,
    db::{create_reply, get_review},
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::{Data, Json, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;
use validation::validate_reply_text;

#[derive(Deserialize)]
pub struct AddReplyRequest {
    text: String,
    review_id: String,
}

/// Allows users to add a reply linked to a review.
#[post("")]
pub async fn add_reply(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    add_reply_request: Json<AddReplyRequest>,
) -> Result<impl Responder> {
    if let Err(err) = validate_reply_text(&add_reply_request.text) {
        return Err(ErrorBadRequest(err));
    }

    if let Err(_) = get_review(&pool, &authenticated_user.0, &add_reply_request.review_id).await {
        return Err(ErrorInternalServerError(
            "unable to find review".to_string(),
        ));
    }

    let reply_res = create_reply(
        &pool,
        &authenticated_user.0,
        &add_reply_request.review_id,
        &add_reply_request.text,
    )
    .await;

    match reply_res {
        Ok(_) => {
            return Ok(HttpResponse::Ok().finish());
        }
        Err(_) => return Err(ErrorInternalServerError("unable create reply".to_string())),
    }
}
