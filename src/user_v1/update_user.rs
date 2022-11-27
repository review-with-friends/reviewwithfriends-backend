use crate::{
    authorization::AuthenticatedUser,
    db::{does_user_exist_by_name, get_user, update_usernames, User},
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::{Data, Json, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    display_name: Option<String>,
    name: Option<String>,
}

#[post("")]
pub async fn update_user(
    authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    update_request: Json<UpdateUserRequest>,
) -> Result<impl Responder> {
    let new_display_name: String;
    let new_name: String;

    let user: User;
    let user_res = get_user(&pool, &authenticated_user.0).await;

    match user_res {
        Ok(_user) => {
            user = _user;
        }
        Err(_) => return Err(ErrorInternalServerError("unable to get user")),
    }

    if update_request.display_name.is_none() && update_request.name.is_none() {
        return Ok(HttpResponse::Ok().finish());
    }

    match &update_request.display_name {
        Some(display_name) => new_display_name = display_name.to_string(),
        None => new_display_name = user.display_name,
    }

    match &update_request.name {
        Some(name) => new_name = name.to_string(),
        None => new_name = user.name,
    }

    let existing_user_res = does_user_exist_by_name(&pool, &new_name).await;

    match existing_user_res {
        Ok(exists) => {
            if exists {
                return Err(ErrorBadRequest("user already exists"));
            }
        }
        Err(_) => return Err(ErrorInternalServerError("failed to get existing users")),
    }

    let update_res =
        update_usernames(&pool, &authenticated_user.0, &new_display_name, &new_name).await;

    match update_res {
        Ok(_) => return Ok(HttpResponse::Ok().finish()),
        Err(_) => return Err(ErrorInternalServerError("failed to update user")),
    }
}
