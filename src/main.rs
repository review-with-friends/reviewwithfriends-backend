use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use auth_routes::*;
use authorization::Authorization;
use friend_routes::*;
use jwt::{encode_jwt_secret, SigningKeys};
use ping_routes::ping;
use sqlx::MySqlPool;
use std::env;

mod auth_routes;
mod authorization;
mod db;
mod friend_routes;
mod ping_routes;

#[derive(Clone)]
pub struct Config {
    twilio_key: String,
    db_connection_string: String,
    signing_keys: SigningKeys,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Config = build_config();
    let pool: MySqlPool = MySqlPool::connect(&config.db_connection_string)
        .await
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(pool.clone()))
            .wrap(Authorization)
            .service(web::scope("/ping").service(ping))
            .service(web::scope("/auth").service(request_code).service(sign_in))
            .service(
                web::scope("/api").service(
                    web::scope("/v1").service(
                        web::scope("/friends")
                            .service(get_friends)
                            .service(get_outgoing_requests)
                            .service(get_incoming_requests)
                            .service(get_incoming_ignored_requests)
                            .service(send_request)
                            .service(accept_request)
                            .service(cancel_request)
                            .service(decline_request)
                            .service(remove_friend),
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
            db_connection_string: String::from("mysql://root:test123@localhost:53208/mob"),
            signing_keys: encode_jwt_secret("thisisatestkey"),
        },
        Err(_) => Config {
            twilio_key: env::var("TWILIO").unwrap(),
            db_connection_string: env::var("DB_CONNECTION").unwrap(),
            signing_keys: encode_jwt_secret(&env::var("JWT_KEY").unwrap()),
        },
    }
}

fn build_binding() -> (&'static str, u16) {
    let is_dev = env::var("MOB_DEV");

    match is_dev {
        Ok(_) => ("127.0.0.1", 8080),
        Err(_) => ("0.0.0.0", 80),
    }
}
