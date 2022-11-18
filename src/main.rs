use actix_web::{
    web::{self, Data},
    App, HttpServer,
};

use auth_routes::*;
use authorization::Authorization;
use friend_v1::{
    accept_friend, add_friend, cancel_friend, decline_friend, get_friends, get_ignored_friends,
    get_incoming_friends, get_outgoing_friends, ignore_friend, remove_friend,
};
use images::create_client;
use jwt::{encode_jwt_secret, SigningKeys};
use ping_routes::{pic, ping, upload_pic};
use sqlx::MySqlPool;
use std::env;

mod auth_routes;
mod authorization;
mod db;
mod friend_v1;
mod ping_routes;

#[derive(Clone)]
pub struct Config {
    twilio_key: String,
    db_connection_string: String,
    signing_keys: SigningKeys,
    spaces_key: String,
    spaces_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Config = build_config();

    let pool: MySqlPool = MySqlPool::connect(&config.db_connection_string)
        .await
        .unwrap();

    let client = create_client(&config.spaces_key, &config.spaces_secret);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(client.clone()))
            .wrap(Authorization)
            .service(
                web::scope("/ping")
                    .service(ping)
                    .service(pic)
                    .service(upload_pic),
            )
            .service(web::scope("/auth").service(request_code).service(sign_in))
            .service(
                web::scope("/api").service(
                    web::scope("/v1").service(
                        web::scope("/friends")
                            .service(get_friends)
                            .service(get_outgoing_friends)
                            .service(get_incoming_friends)
                            .service(get_ignored_friends)
                            .service(add_friend)
                            .service(accept_friend)
                            .service(cancel_friend)
                            .service(decline_friend)
                            .service(remove_friend)
                            .service(ignore_friend),
                    ),
                ),
            )
    })
    .bind(build_binding())?
    .run()
    .await
}

fn build_config() -> Config {
    let is_dev = env::var("MOB_DEV");

    match is_dev {
        Ok(_) => Config {
            twilio_key: String::from("123"),
            db_connection_string: String::from("mysql://root:test123@localhost:55324/mob"),
            signing_keys: encode_jwt_secret("thisisatestkey"),
            spaces_key: env::var("MOB_SPACES_KEY").unwrap(),
            spaces_secret: env::var("MOB_SPACES_SECRET").unwrap(),
        },
        Err(_) => Config {
            twilio_key: env::var("TWILIO").unwrap(),
            db_connection_string: env::var("DB_CONNECTION").unwrap(),
            signing_keys: encode_jwt_secret(&env::var("JWT_KEY").unwrap()),
            spaces_key: env::var("SPACES_KEY").unwrap(),
            spaces_secret: env::var("SPACES_SECRET").unwrap(),
        },
    }
}

fn build_binding() -> (&'static str, u16) {
    let is_dev = env::var("MOB_DEV");

    match is_dev {
        Ok(_) => ("127.0.0.1", 8081),
        Err(_) => ("0.0.0.0", 80),
    }
}
