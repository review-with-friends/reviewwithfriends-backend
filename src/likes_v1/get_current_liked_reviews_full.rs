use crate::{
    authorization::AuthenticatedUser,
    compound_types::CompoundReviewPub,
    db::get_liked_reviews,
    review_v1::{gather_compound_review, ReviewPub},
    tracing::add_error_span,
};
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
#[get("/current_full")]
pub async fn get_current_liked_reviews_full(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    get_paged_current_likes: Query<GetPagedCurrentLikes>,
) -> Result<impl Responder> {
    let reviews_res =
        get_liked_reviews(&pool, &authenticated_user.0, get_paged_current_likes.page).await;

    let mut compound_reviews: Vec<CompoundReviewPub> = vec![];

    match reviews_res {
        Ok(reviews) => {
            let reviews_pub: Vec<ReviewPub> = reviews
                .into_iter()
                .map(|f| -> ReviewPub { f.into() })
                .collect();

            for review_pub in reviews_pub.into_iter() {
                let compound_review_res =
                    gather_compound_review(&pool, &authenticated_user.0, review_pub).await;

                match compound_review_res {
                    Ok(compound_review) => compound_reviews.push(compound_review),
                    Err(_) => {
                        return Err(ErrorInternalServerError(
                            "failed gathering review contents".to_string(),
                        ))
                    }
                }
            }
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError("unable to get likes".to_string()));
        }
    }

    return Ok(Json(compound_reviews));
}
