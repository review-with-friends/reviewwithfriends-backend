use crate::{
    authorization::AuthenticatedUser,
    db::{get_review, update_review, Review},
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    post,
    web::{Data, Json, ReqData},
    HttpResponse, Responder, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validation::validate_reply_text;

#[derive(Deserialize, Serialize)]
pub struct EditReviewRequest {
    pub review_id: String,
    pub text: Option<String>,
    pub stars: Option<u8>,
}

#[post("/edit")]
pub async fn edit_review(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    edit_review_request: Json<EditReviewRequest>,
) -> Result<impl Responder> {
    let new_stars: u8;
    let new_text: String;

    let review: Review;
    let review_res = get_review(&pool, &authenticated_user.0, &edit_review_request.review_id).await;

    match review_res {
        Ok(review_opt) => {
            if let Some(review_tmp) = review_opt {
                review = review_tmp;
            } else {
                return Err(ErrorNotFound("could not find review"));
            }
        }
        Err(_) => {
            return Err(ErrorInternalServerError("failed to get review"));
        }
    }

    if review.user_id != authenticated_user.0 {
        return Ok(HttpResponse::BadRequest().body("unable to edit this review"));
    }

    if edit_review_request.stars.is_none() && edit_review_request.text.is_none() {
        return Ok(HttpResponse::Ok().finish());
    }

    match edit_review_request.stars {
        Some(stars) => new_stars = stars,
        None => new_stars = review.stars,
    }

    match &edit_review_request.text {
        Some(text) => new_text = text.to_string(),
        None => new_text = review.text,
    }

    if let Err(validation_error) = validate_reply_text(&new_text) {
        return Err(ErrorInternalServerError(validation_error));
    }

    let update_res =
        update_review(&pool, &edit_review_request.review_id, new_stars, &new_text).await;

    match update_res {
        Ok(_) => {
            return Ok(HttpResponse::Ok().finish());
        }
        Err(_) => {
            return Err(ErrorInternalServerError("failed to edit review"));
        }
    }
}
