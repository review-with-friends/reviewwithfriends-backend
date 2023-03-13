use crate::{
    db::{
        create_authattempt, get_current_phoneauths, get_phoneauth_attempts, get_user_by_phone,
        update_authattempt_used, update_user_phone, PhoneAuth, User,
    },
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
    /// Old Account Phone Number.
    phone: String,
    /// New Account Phone Number.
    new_phone: String,
    /// Code to give you access to swap numbers.
    code: String,
}

/// Returns the user JWT for future requests.
///
/// The passed phone, new_phone, and code are validated.
///
/// We validate rate constraints with the persistent auth attempt records.
#[post("/update_phone")]
pub async fn update_phone(
    config: Data<Config>,
    pool: Data<MySqlPool>,
    sign_in_request: Query<SignInRequest>,
) -> Result<impl Responder> {
    if &sign_in_request.phone == &sign_in_request.new_phone {
        return Err(ErrorBadRequest("phone numbers are identical"));
    }

    let valid_phone = validation::validate_phone(&sign_in_request.phone);
    if let Err(phone_err) = valid_phone {
        return Err(ErrorBadRequest(phone_err.to_string()));
    }

    let valid_new_phone = validation::validate_phone(&sign_in_request.new_phone);
    if let Err(phone_err) = valid_new_phone {
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
    if let Ok(phone_auth_attempts) = phone_auth_attemps_res {
        if phone_auth_attempts.len() >= 4 {
            return Err(ErrorBadRequest(
                "too many auth attempts - wait a bit before trying again",
            ));
        }
    } else {
        return Err(ErrorInternalServerError("unable to get auth attempts"));
    }

    let phone_auth_res = get_current_phoneauths(&pool, &sign_in_request.phone).await;
    let phone_auths: Vec<PhoneAuth>;
    if let Ok(phone_auths_tmp) = phone_auth_res {
        phone_auths = phone_auths_tmp;
    } else {
        return Err(ErrorInternalServerError("unable to get current phoneauths"));
    }

    let matched_phoneauth = phone_auths
        .iter()
        .filter(|ar| ar.code == sign_in_request.code)
        .collect::<Vec<&PhoneAuth>>();

    if matched_phoneauth.len() == 1 {
        let old_user: User;
        let user_res = get_user_by_phone(&pool, &sign_in_request.phone).await;
        if let Ok(user_opt) = user_res {
            if let Some(user_tmp) = user_opt {
                old_user = user_tmp;
            } else {
                return Err(ErrorInternalServerError(
                    "unable to find user for given phone",
                ));
            }
        } else {
            return Err(ErrorInternalServerError("error fetching user by phone"));
        }

        let new_user_res = get_user_by_phone(&pool, &sign_in_request.new_phone).await;
        if let Ok(new_user_opt) = new_user_res {
            if let Some(user_tmp) = new_user_opt {
                // remove phone number from new user if it exists
                let res = update_user_phone(&pool, &user_tmp.id, "").await;
                if let Err(_) = res {
                    return Err(ErrorInternalServerError("error clearing existing account"));
                }
            }
        } else {
            return Err(ErrorInternalServerError("error fetching user by phone"));
        }

        let res = update_user_phone(&pool, &old_user.id, &sign_in_request.new_phone).await;
        if let Err(_) = res {
            return Err(ErrorInternalServerError(
                "error updating account phone number",
            ));
        }

        let authattempt_update_res =
            update_authattempt_used(&pool, &matched_phoneauth.first().unwrap().id).await;
        if let Err(_) = authattempt_update_res {
            return Err(ErrorInternalServerError("unable to update authattempt"));
        }

        let jwt = mint_jwt(&config.signing_keys, &old_user.id);

        Ok(jwt)
    } else {
        return Err(ErrorBadRequest("invalid code"));
    }
}
