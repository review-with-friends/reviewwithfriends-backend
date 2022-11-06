use std::{collections::HashMap, time::Duration};

use crate::{
    db::{
        create_authattempt, create_phoneauth, create_user, get_current_phoneauths,
        get_phoneauth_attempts, get_user_by_phone, update_authattempt_used, PhoneAuth, User,
    },
    Config,
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::Data,
    HttpResponse, Responder, Result,
};
use chrono::Utc;
use jwt::mint_jwt;
use rand::Rng;
use reqwest::{ClientBuilder, StatusCode};
use sqlx::MySqlPool;
use uuid::Uuid;
use validation;

#[post("/requestcode?<phone>")]
pub async fn request_code(
    pool: Data<MySqlPool>,
    config: Data<Config>,
    phone: String,
) -> Result<impl Responder> {
    let valid_phone = validation::validate_phone(&phone);
    if let Err(phone_err) = valid_phone {
        return Err(ErrorBadRequest(phone_err));
    }

    let phoneauths_res = get_current_phoneauths(&pool, &phone).await;

    match phoneauths_res {
        Ok(phoneauths) => {
            if phoneauths.len() >= 3 {
                return Err(ErrorBadRequest("too many auth attempts"));
            }
        }
        Err(_) => return Err(ErrorInternalServerError("unable to fetch auths")),
    }

    let user_res = get_user_by_phone(&pool, &phone).await;

    let existing_user: User;

    match user_res {
        Ok(user_opt) => {
            if let Some(user) = user_opt {
                existing_user = user;
            } else {
                existing_user = User {
                    id: Uuid::new_v4().to_string(),
                    name: get_new_user_name(),
                    display_name: "".to_string(),
                    phone: phone.to_string(),
                    created: Utc::now().naive_utc(),
                };

                let create_res = create_user(&pool, &existing_user).await;

                match create_res {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(ErrorInternalServerError("error creating user"));
                    }
                }
            }
        }
        Err(_) => {
            return Err(ErrorInternalServerError("error fetching user"));
        }
    };

    let auth_code = get_new_auth_code();
    let phoneauth_res = create_phoneauth(&pool, &existing_user.phone, &auth_code).await;

    match phoneauth_res {
        Ok(_) => {}
        Err(_) => {
            return Err(ErrorInternalServerError("error creating auth"));
        }
    }

    let auth_res = send_auth(&config.twilio_key, &existing_user.phone, &auth_code).await;

    match auth_res {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(_) => Ok(HttpResponse::InternalServerError()),
    }
}

#[post("/signin?<code>&<phone>")]
pub async fn sign_in(
    config: Data<Config>,
    pool: Data<MySqlPool>,
    code: String,
    phone: String,
) -> Result<impl Responder> {
    let valid_phone = validation::validate_phone(&phone);
    if let Err(phone_err) = valid_phone {
        return Err(ErrorBadRequest(phone_err.to_string()));
    }

    let valid_code = validation::validate_code(&code);
    if let Err(code_err) = valid_code {
        return Err(ErrorBadRequest(code_err.to_string()));
    }

    let create_authattempt_res = create_authattempt(&pool, &phone).await;
    if let Err(_) = create_authattempt_res {
        return Err(ErrorInternalServerError("unable to start auth attempt"));
    }

    let phone_auth_attemps_res = get_phoneauth_attempts(&pool, &phone).await;
    if let Ok(phone_auth_attempts) = phone_auth_attemps_res {
        if phone_auth_attempts.len() >= 4 {
            return Err(ErrorBadRequest("too many auth attempts"));
        }
    } else {
        return Err(ErrorInternalServerError("unable to get auth attempts"));
    }

    let phone_auth_res = get_current_phoneauths(&pool, &phone).await;
    let phone_auths: Vec<PhoneAuth>;
    if let Ok(phone_auths_tmp) = phone_auth_res {
        phone_auths = phone_auths_tmp;
    } else {
        return Err(ErrorInternalServerError("unable to get current phoneauths"));
    }

    let matched_phoneauth = phone_auths
        .iter()
        .filter(|ar| ar.code == code)
        .collect::<Vec<&PhoneAuth>>();

    if matched_phoneauth.len() == 1 {
        let user: User;
        let user_res = get_user_by_phone(&pool, &phone).await;
        if let Ok(user_opt) = user_res {
            if let Some(user_tmp) = user_opt {
                user = user_tmp;
            } else {
                return Err(ErrorInternalServerError(
                    "unable to find use for given phone",
                ));
            }
        } else {
            return Err(ErrorInternalServerError("error fetching user by phone"));
        }

        let authattempt_update_res =
            update_authattempt_used(&pool, &matched_phoneauth.first().unwrap().id).await;
        if let Err(_) = authattempt_update_res {
            return Err(ErrorInternalServerError("unable to update authattempt"));
        }

        let jwt = mint_jwt(&config.signing_keys, &user.id);

        Ok(jwt)
    } else {
        return Err(ErrorBadRequest("invalid code"));
    }
}

async fn send_auth(twilio_secret: &String, phone: &str, code: &str) -> Result<(), String> {
    let request_url = "https://api.twilio.com/2010-04-01/Accounts/AC0094c61aa39fc9c673130f6e28e43bad/Messages.json";

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build().unwrap();

    let clean_phone = phone.to_string().replace(" ", "");

    let mut params = HashMap::new();
    params.insert("Body", format!("{} is your Mob auth code!", code));
    params.insert("From", "+17246134841".to_string());
    params.insert("To", format!("+{}", clean_phone));

    let response_res = client
        .post(request_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .basic_auth("AC0094c61aa39fc9c673130f6e28e43bad", Some(twilio_secret))
        .send()
        .await;

    match response_res {
        Ok(response) => match response.status() {
            StatusCode::OK => Ok(()),
            _ => Err(format!(
                "{} {}",
                "twilio send failed with status".to_string(),
                response.status()
            )),
        },
        Err(_) => Err("twilio send failed".to_string()),
    }
}

/// Gets a name for a new user to default to.
/// The user is expected to be able to set this to anything not already taken.
fn get_new_user_name() -> String {
    let mut rng = rand::thread_rng();
    let mut user_name = String::from("newuser");

    for _ in 0..9 {
        let num = rng.gen_range(0..9);
        user_name.push(char::from_u32(num).unwrap());
    }

    return user_name;
}

/// THIS IS TEMPORARY AND NEEDS TO BE CHANGED TO SOMETHING CRYPTOGRAPHICALLY SECURE
/// COLTON FUCKING MAKE THIS GOOD BEFORE BETA
fn get_new_auth_code() -> String {
    let mut rng = rand::thread_rng();
    let mut user_name = String::from("");

    for _ in 0..9 {
        let num = rng.gen_range(0..9);
        user_name.push_str(&num.to_string());
    }

    return user_name;
}
