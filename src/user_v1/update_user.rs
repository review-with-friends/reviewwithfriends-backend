use crate::{
    authorization::AuthenticatedUser,
    db::{does_user_exist_by_name, get_user, update_usernames, User},
    tracing::add_error_span,
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound},
    post,
    web::{Data, Json, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;
use sqlx::MySqlPool;
use validation::{validate_display_name, validate_name};

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    display_name: Option<String>,
    name: Option<String>,
}

/// Allows the updating of display_name and name fields.
/// display_name isn't unique in the table; but name is.
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
        Ok(user_opt) => {
            if let Some(user_tmp) = user_opt {
                user = user_tmp;
            } else {
                return Err(ErrorNotFound("could not find user"));
            }
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError("unable to get user"));
        }
    }

    if update_request.display_name.is_none() && update_request.name.is_none() {
        return Ok(HttpResponse::Ok().finish());
    }

    match &update_request.display_name {
        Some(display_name) => new_display_name = display_name.to_string(),
        None => new_display_name = user.display_name,
    }

    match &update_request.name {
        Some(name) => new_name = name.to_lowercase(),
        None => new_name = user.name.clone(),
    }

    if let Err(err) = validate_display_name(&new_display_name) {
        return Err(ErrorBadRequest(err.to_string()));
    }

    if let Err(err) = validate_name(&new_name) {
        return Err(ErrorBadRequest(err.to_string()));
    }

    if new_name != user.name {
        let existing_user_res = does_user_exist_by_name(&pool, &new_name).await;

        match existing_user_res {
            Ok(exists) => {
                if exists {
                    return Err(ErrorBadRequest("user already exists"));
                }
            }
            Err(error) => {
                add_error_span(&error);
                return Err(ErrorInternalServerError("failed to get existing users"));
            }
        }
    }

    let update_res =
        update_usernames(&pool, &authenticated_user.0, &new_display_name, &new_name).await;

    match update_res {
        Ok(_) => return Ok(HttpResponse::Ok().finish()),
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError("failed to update user"));
        }
    }
}
