#[macro_use]
extern crate rocket;

use ::serde::Deserialize;
use db::DBClient;
use jwt::{encode_jwt_secret, validate_jwt, SigningKeys};
use rocket::{
    fairing::AdHoc,
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use rocket_db_pools::Database;

mod auth_routes;
mod db;
mod test_routes;

#[derive(Deserialize)]
pub struct Config {
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
        .mount("/api/test", routes![test_routes::auth_hello_world])
        .mount(
            "/auth",
            routes![auth_routes::request_code, auth_routes::sign_in],
        )
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
