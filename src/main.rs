#[macro_use]
extern crate rocket;

use ::serde::Deserialize;
use db::query::DBClient;
use jwt::{encode_jwt_secret, validate_jwt, SigningKeys};
use reqwest::ClientBuilder;
use rocket::{
    fairing::AdHoc,
    http::Status,
    request::{FromRequest, Outcome},
    response::status,
    Request, State,
};
use rocket_db_pools::{
    sqlx::{self, Row},
    Connection, Database,
};
use std::{collections::HashMap, time::Duration};

mod auth_routes;
mod db;
mod test_routes;

#[derive(Deserialize)]
struct Config {
    twilio: String,
}

#[launch]
async fn rocket() -> _ {
    println!("Mob has started.");

    let rocket = rocket::build();
    let figment = rocket.figment();

    let custom: String = figment
        .extract_inner("jwt_secret")
        .expect("missing jwt secret");
    let signing_keys = encode_jwt_secret(&custom);

    rocket
        .attach(DBClient::init())
        .attach(AdHoc::config::<Config>())
        .manage(signing_keys)
        .mount(
            "/api/test",
            routes![
                test_routes::hello_world,
                phone_auth,
                test_routes::auth_hello_world
            ],
        )
        .mount(
            "/auth",
            routes![auth_routes::request_code, auth_routes::sign_in],
        )
}

#[get("/phoneauth")]
async fn phone_auth(
    mut _client: Connection<DBClient>,
    config: &State<Config>,
) -> status::Custom<String> {
    let request_url = "https://api.twilio.com/2010-04-01/Accounts/AC0094c61aa39fc9c673130f6e28e43bad/Messages.json";

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build().unwrap();

    let mut params = HashMap::new();
    params.insert("Body", "Hello from Twilio");
    params.insert("From", "+17246134841");
    params.insert("To", "+17014910059");

    let response = client
        .post(request_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .basic_auth("AC0094c61aa39fc9c673130f6e28e43bad", Some(&config.twilio))
        .send()
        .await
        .unwrap();
    return status::Custom(Status::Ok, String::from(response.text().await.unwrap()));
}

#[derive(Debug)]
pub enum JWTError {
    Invalid,
}

pub struct JWTAuthorized(pub String);

/// When JWTAuthorized is on the route, this guard will fire and ensure authorization is passed.
/// JWTAuthorized contains the users id for fetching information and validation relationships.
#[async_trait]
impl<'r> FromRequest<'r> for JWTAuthorized {
    type Error = JWTError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let jwt = request.headers().get_one("Authorization").unwrap_or("");
        let keys = request.rocket().state::<SigningKeys>().unwrap();
        match validate_jwt(&keys, jwt) {
            Some(id) => {
                return Outcome::Success(JWTAuthorized(id));
            }
            None => {
                return Outcome::Failure((Status::Unauthorized, JWTError::Invalid));
            }
        }
    }
}
