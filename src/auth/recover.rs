use crate::{
    db::{create_phoneauth, get_current_phoneauths, get_user_by_phone, User},
    Config,
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::{Data, Query},
    HttpResponse, Result,
};
use opentelemetry::{
    global,
    trace::{Span, Status, Tracer},
};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validation;

use super::get_new_auth_code;

#[derive(Deserialize)]
pub struct RequestCodeRequest {
    phone: String,
}

/// Endpoint for requesting an auth code for recovery.
/// The auth code is sent to the accounts email.
#[post("/recovery_code")]
pub async fn recovery_code(
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
        Err(_) => return Err(ErrorInternalServerError("unable to fetch auths")),
    }

    let user_res = get_user_by_phone(&pool, &request_code_request.phone).await;

    let existing_user: User;

    match user_res {
        Ok(user_opt) => {
            if let Some(user) = user_opt {
                existing_user = user;
            } else {
                return Err(ErrorBadRequest("user doesn't exist"));
            }
        }
        Err(_) => {
            return Err(ErrorInternalServerError("error fetching user"));
        }
    };

    let existing_email: String;
    if let Some(email) = existing_user.email {
        existing_email = email;
    } else {
        return Err(ErrorBadRequest(
            "recovery email not setup, please contact support.",
        ));
    }

    let auth_code = get_new_auth_code();
    let phoneauth_res = create_phoneauth(&pool, &existing_user.phone, &auth_code).await;

    match phoneauth_res {
        Ok(_) => {}
        Err(_) => {
            return Err(ErrorInternalServerError("error creating auth"));
        }
    }

    let tracer = global::tracer("exception");
    let mut span = tracer.start("email auth failure");

    let auth_res = send_auth_email(
        &http_client,
        &config.sendgrid_key,
        &existing_email,
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
            span.end();
            return Ok(HttpResponse::InternalServerError().body("failed to send auth request"));
        }
    }
}

/// Sends an email with the generated auth code to the users phone.
async fn send_auth_email(
    client: &Client,
    secret: &String,
    email: &str,
    code: &str,
) -> Result<(), String> {
    const REQUEST_URL: &str = "https://api.sendgrid.com/v3/mail/send";

    let email_obj = SendGridEmail {
        from: SendGridRecipient {
            name: "BeLocal Auth".to_string(),
            email: "auth@em9516.spacedoglabs.com".to_string(),
        },
        subject: "BeLocal Account Recovery Code".to_string(),
        content: vec![SendGridContentItem {
            content_type: "text/html".to_string(),
            value: format!(
                "<p>Here is your BeLocal account recovery code: {}</p>",
                code
            ),
        }],
        personalizations: vec![SendGridPersonalizations {
            to: vec![SendGridRecipient {
                name: "".to_string(),
                email: email.to_string(),
            }],
        }],
    };

    let body: String;

    if let Ok(body_ok) = serde_json::to_string(&email_obj) {
        body = body_ok;
    } else {
        return Err("failed to create email body".to_string());
    }

    let response_res = client
        .post(REQUEST_URL)
        .header("Content-Type", "application/json")
        .header("authorization", format!("bearer {}", &secret))
        .body(body)
        .send()
        .await;

    match response_res {
        Ok(response) => match response.status() {
            StatusCode::ACCEPTED => Ok(()),
            _ => {
                println!("{}", response.text().await.unwrap());
                return Err(format!(
                    "{}",
                    "sendgrid email send finished with unexpected status:".to_string()
                ));
            }
        },
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Serialize)]
struct SendGridEmail {
    pub from: SendGridRecipient,
    pub subject: String,
    pub content: Vec<SendGridContentItem>,
    pub personalizations: Vec<SendGridPersonalizations>,
}

#[derive(Serialize)]
struct SendGridRecipient {
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
struct SendGridPersonalizations {
    pub to: Vec<SendGridRecipient>,
}

#[derive(Serialize)]
struct SendGridContentItem {
    #[serde(rename = "type")]
    pub content_type: String,
    pub value: String,
}
