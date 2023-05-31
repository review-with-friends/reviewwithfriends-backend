use crate::{
    db::{
        create_authattempt, get_current_phoneauths, get_phoneauth_attempts, get_user_by_phone,
        update_authattempt_used, PhoneAuth, User,
    },
    tracing::add_error_span,
    Config,
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::{Data, Query},
    Responder, Result,
};
use jwt::mint_jwt;
use serde::Deserialize;
use sqlx::MySqlPool;
use validation;

#[derive(Deserialize)]
pub struct SignInRequest {
    phone: String,
    code: String,
}

/// Returns the user JWT for future requests.
///
/// The passed phone and code are validated.
///
/// We validate rate constraints with the persistent auth attempt records.
#[post("/signin")]
pub async fn sign_in(
    config: Data<Config>,
    pool: Data<MySqlPool>,
    sign_in_request: Query<SignInRequest>,
) -> Result<impl Responder> {
    let valid_phone = validation::validate_phone(&sign_in_request.phone);
    if let Err(phone_err) = valid_phone {
        return Err(ErrorBadRequest(phone_err.to_string()));
    }

    let valid_code = validation::validate_code(&sign_in_request.code);
    if let Err(code_err) = valid_code {
        return Err(ErrorBadRequest(code_err.to_string()));
    }

    let create_authattempt_res = create_authattempt(&pool, &sign_in_request.phone).await;
    if let Err(_) = create_authattempt_res {
        return Err(ErrorInternalServerError("unable to start auth attempt"));
    }

    let phone_auth_attemps_res = get_phoneauth_attempts(&pool, &sign_in_request.phone).await;
    match phone_auth_attemps_res {
        Ok(phone_auth_attempts) => {
            if phone_auth_attempts.len() >= 4 {
                return Err(ErrorBadRequest(
                    "too many auth attempts - wait a bit before trying again",
                ));
            }
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError("unable to get auth attempts"));
        }
    }

    let phone_auth_res = get_current_phoneauths(&pool, &sign_in_request.phone).await;
    let phone_auths: Vec<PhoneAuth>;

    match phone_auth_res {
        Ok(phone_auths_tmp) => {
            phone_auths = phone_auths_tmp;
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError("unable to get current phoneauths"));
        }
    }

    let matched_phoneauth = phone_auths
        .iter()
        .filter(|ar| ar.code == sign_in_request.code)
        .collect::<Vec<&PhoneAuth>>();

    if matched_phoneauth.len() == 1 {
        let user: User;
        let user_res = get_user_by_phone(&pool, &sign_in_request.phone).await;

        match user_res {
            Ok(user_opt) => {
                if let Some(user_tmp) = user_opt {
                    user = user_tmp;
                } else {
                    return Err(ErrorInternalServerError("unable to find user"));
                }
            }
            Err(error) => {
                add_error_span(&error);
                return Err(ErrorInternalServerError("error fetching user"));
            }
        }

        let authattempt_update_res =
            update_authattempt_used(&pool, &matched_phoneauth.first().unwrap().id).await;
        if let Err(error) = authattempt_update_res {
            add_error_span(&error);
            return Err(ErrorInternalServerError("unable to update authattempt"));
        }

        let jwt = mint_jwt(&config.signing_keys, &user.id);

        Ok(jwt)
    } else {
        return Err(ErrorBadRequest("invalid code"));
    }
}

#[post("/signin-demo")]
pub async fn sign_in_demo(config: Data<Config>) -> Result<impl Responder> {
    let jwt = mint_jwt(&config.signing_keys, "226f982d-1971-4085-a8a8-bc0074de0b84");

    Ok(jwt)
}
