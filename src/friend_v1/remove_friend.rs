use crate::{
    authorization::AuthenticatedUser,
    db::{get_current_friends, remove_current_friend},
    tracing::add_error_span,
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::{Data, Query, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct RemoveRequest {
    friend_id: String,
}

/// Removes a friend from a users friendlist.
#[post("/remove")]
pub async fn remove_friend(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    remove_request: Query<RemoveRequest>,
) -> Result<impl Responder> {
    if authenticated_user
        .0
        .eq_ignore_ascii_case(&remove_request.friend_id)
    {
        return Err(ErrorBadRequest("you cannot unfriend yourself"));
    }

    let friends_res = get_current_friends(&pool, &authenticated_user.0.clone()).await;

    match friends_res {
        Ok(friends) => {
            let friend_exists = friends.into_iter().any(|fr| -> bool {
                return fr.friend_id == remove_request.friend_id;
            });

            if friend_exists {
                let remove_res = remove_current_friend(
                    &pool,
                    &&authenticated_user.0.clone(),
                    &remove_request.friend_id,
                )
                .await;

                match remove_res {
                    Ok(_) => Ok(HttpResponse::Ok()),
                    Err(error) => {
                        add_error_span(&error);
                        Err(ErrorInternalServerError("failed removing friend"))
                    }
                }
            } else {
                return Err(ErrorBadRequest("friend doesnt exist"));
            }
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError("could not fetch friends"));
        }
    }
}
