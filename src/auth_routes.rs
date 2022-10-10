use std::{collections::HashMap, time::Duration};

use crate::{
    db::{
        create_authattempt, create_phoneauth, create_user, get_current_phoneauths,
        get_phoneauth_attempts, get_user_by_phone, update_authattempt_used, AuthAttempt, PhoneAuth,
        User,
    },
    Config, DBClient,
};
use chrono::Utc;
use jwt::{mint_jwt, SigningKeys};
use rand::Rng;
use reqwest::ClientBuilder;
use rocket::State;
use uuid::Uuid;
use validation;

#[post("/requestcode?<phone>")]
pub async fn request_code(
    client: &DBClient,
    config: &State<Config>,
    phone: &str,
) -> Result<(), String> {
    validation::validate_phone(phone)?;
    let phoneauths_res = get_current_phoneauths(client, phone.to_string()).await;

    match phoneauths_res {
        Ok(phoneauths) => {
            if phoneauths.len() >= 3 {
                return Err("too many auth attempts".to_string());
            }
        }
        Err(_) => return Err("unable to fetch auths".to_string()),
    }

    let user_res = get_user_by_phone(client, phone.to_string()).await;

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

                let create_res = create_user(client, &existing_user).await;

                match create_res {
                    Ok(_) => {}
                    Err(_) => return Err("error creating user".to_string()),
                }
            }
        }
        Err(_) => return Err("error fetching from db".to_string()),
    };

    let auth_code = get_new_auth_code();
    let phoneauth_res = create_phoneauth(client, &existing_user.phone, &auth_code).await;

    match phoneauth_res {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    send_auth(&config.twilio, &existing_user.phone, &auth_code).await;

    Ok(())
}

#[post("/signin?<code>&<phone>")]
pub async fn sign_in(
    signing_keys: &State<SigningKeys>,
    client: &DBClient,
    code: &str,
    phone: &str,
) -> Result<String, String> {
    validation::validate_code(code)?;
    validation::validate_phone(phone)?;

    let create_authattempt_res = create_authattempt(client, phone, code).await;
    if let Err(_) = create_authattempt_res {
        return Err("unable to start auth attempt".to_string());
    }

    let phone_auth_attemps_res = get_phoneauth_attempts(client, phone.to_string()).await;
    if let Ok(phone_auth_attempts) = phone_auth_attemps_res {
        if phone_auth_attempts.len() >= 4 {
            return Err("too many auth attempts".to_string());
        }
    } else {
        return Err("unable to fetch auth attempts".to_string());
    }

    let phone_auth_res = get_current_phoneauths(client, phone.to_string()).await;
    let phone_auths: Vec<PhoneAuth>;
    if let Ok(phone_auths_tmp) = phone_auth_res {
        phone_auths = phone_auths_tmp;
    } else {
        return Err("unable to fetch auths".to_string());
    }

    let matched_phoneauth = phone_auths
        .iter()
        .filter(|ar| ar.code == code)
        .collect::<Vec<&PhoneAuth>>();

    if matched_phoneauth.len() == 1 {
        let user: User;
        let user_res = get_user_by_phone(client, phone.to_string()).await;
        if let Ok(user_opt) = user_res {
            if let Some(user_tmp) = user_opt {
                user = user_tmp;
            } else {
                return Err("unable to find user for phone number".to_string());
            }
        } else {
            return Err("error fetching user by phone".to_string());
        }

        let authattempt_update_res =
            update_authattempt_used(client, &matched_phoneauth.first().unwrap().id).await;
        if let Err(_) = authattempt_update_res {
            return Err("unable to update authattempt".to_string());
        }

        let jwt = mint_jwt(&signing_keys, &user.id);

        Ok(jwt)
    } else {
        Err("invalid code".to_string())
    }
}

async fn send_auth(twilio_secret: &String, phone: &str, code: &str) {
    let request_url = "https://api.twilio.com/2010-04-01/Accounts/AC0094c61aa39fc9c673130f6e28e43bad/Messages.json";

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build().unwrap();

    let clean_phone = phone.to_string().replace(" ", "");

    let mut params = HashMap::new();
    params.insert("Body", format!("{} is your Mob auth code!", code));
    params.insert("From", "+17246134841".to_string());
    params.insert("To", clean_phone);

    let response = client
        .post(request_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .basic_auth("AC0094c61aa39fc9c673130f6e28e43bad", Some(twilio_secret))
        .send()
        .await
        .unwrap();
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
