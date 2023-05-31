use std::collections::HashMap;

use crate::{
    db::{create_phoneauth, create_user, get_current_phoneauths, get_user_by_phone, User},
    tracing::add_error_span,
    Config,
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::{Data, Query},
    HttpResponse, Result,
};
use chrono::Utc;
use opentelemetry::{
    global,
    trace::{Span, Status, Tracer},
};
use rand::Rng;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use sqlx::MySqlPool;
use uuid::Uuid;
use validation;

use super::get_new_auth_code;

#[derive(Deserialize)]
pub struct RequestCodeRequest {
    phone: String,
}

/// Endpoint for requesting an auth code.
/// The auth code is sent to the request phone number.
/// We track when we send a code, and try to prevent abuse
/// limiting the frequency on a phone by phone basis
///
/// We may want to add IP limits here as well. SMS is expensive
/// when abused, and can be annoying for targets of said abuse.
#[post("/requestcode")]
pub async fn request_code(
    pool: Data<MySqlPool>,
    config: Data<Config>,
    http_client: Data<Client>,
    request_code_request: Query<RequestCodeRequest>,
) -> Result<HttpResponse> {
    let valid_phone = validation::validate_phone(&request_code_request.phone);
    if let Err(phone_err) = valid_phone {
        return Err(ErrorBadRequest(phone_err));
    }

    let phoneauths_res = get_current_phoneauths(&pool, &request_code_request.phone).await;

    match phoneauths_res {
        Ok(phoneauths) => {
            if phoneauths.len() >= 3 {
                return Err(ErrorBadRequest("too many auth attempts"));
            }
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError("unable to fetch auths"));
        }
    }

    let user_res = get_user_by_phone(&pool, &request_code_request.phone).await;

    let existing_user: User;

    match user_res {
        Ok(user_opt) => {
            if let Some(user) = user_opt {
                existing_user = user;
            } else {
                let new_username = get_new_user_name();
                existing_user = User {
                    id: Uuid::new_v4().to_string(),
                    name: new_username.clone(),
                    display_name: new_username.clone(),
                    phone: request_code_request.phone.to_string(),
                    created: Utc::now().naive_utc(),
                    pic_id: "default".to_string(),
                    device_token: None,
                    email: None,
                    disabled: 0,
                };

                let create_res = create_user(&pool, &existing_user).await;

                match create_res {
                    Ok(_) => {}
                    Err(error) => {
                        add_error_span(&error);
                        return Err(ErrorInternalServerError("error creating user"));
                    }
                }
            }
        }
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError("error fetching user"));
        }
    };

    if existing_user.disabled == 1 {
        return Err(ErrorBadRequest("user is disabled"));
    }

    let auth_code = get_new_auth_code();
    let phoneauth_res = create_phoneauth(&pool, &existing_user.phone, &auth_code).await;

    match phoneauth_res {
        Ok(_) => {}
        Err(error) => {
            add_error_span(&error);
            return Err(ErrorInternalServerError("error creating auth"));
        }
    }

    let tracer = global::tracer("exception");
    let mut span = tracer.start("phone auth failure");

    let auth_res = send_auth(
        &http_client,
        &config.twilio_key,
        &existing_user.phone,
        &auth_code,
    )
    .await;

    match auth_res {
        Ok(_) => {
            span.end();
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => {
            span.set_status(Status::error(err.clone()));
            return Ok(HttpResponse::InternalServerError().body("failed to send auth request"));
        }
    }
}

/// Sends a text with the generated auth code to the users phone.
async fn send_auth(
    client: &Client,
    twilio_secret: &String,
    phone: &str,
    code: &str,
) -> Result<(), String> {
    const REQUEST_URL: &str = "https://api.twilio.com/2010-04-01/Accounts/AC0094c61aa39fc9c673130f6e28e43bad/Messages.json";

    let mut params = HashMap::new();
    params.insert(
        "Body",
        format!(
            "Welcome to Review with friends! Here is your verification code: {} ",
            code
        ),
    );
    params.insert("From", "+17246134841".to_string());
    params.insert("To", format!("+{}", phone));

    let response_res = client
        .post(REQUEST_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .basic_auth("AC0094c61aa39fc9c673130f6e28e43bad", Some(twilio_secret))
        .send()
        .await;

    match response_res {
        Ok(response) => match response.status() {
            StatusCode::CREATED => Ok(()),
            _ => Err(format!(
                "{} {}",
                "twilio send finished with unexpected status:".to_string(),
                response.status()
            )),
        },
        Err(err) => Err(err.to_string()),
    }
}

/// Gets a name for a new user to default to.
/// The user is expected to be able to set this to anything not already taken.
fn get_new_user_name() -> String {
    let mut rng = rand::thread_rng();
    let mut user_name = String::from("newuser");

    for _ in 0..9 {
        let num = rng.gen_range(0..9);
        user_name.push_str(&num.to_string());
    }

    return user_name;
}
