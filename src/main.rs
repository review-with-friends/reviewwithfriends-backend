use actix_web::{
    web::{self, Data, PayloadConfig},
    App, HttpServer,
};
use auth::*;
use authorization::Authorization;
use friend_v1::{
    accept_friend, add_friend, cancel_friend, decline_friend, get_friends, get_ignored_friends,
    get_incoming_friends, get_outgoing_friends, ignore_friend, remove_friend,
};
use images::create_s3_client;
use jwt::{encode_jwt_secret, SigningKeys};
use likes_v1::{get_likes, like_review, unlike_review};
use opentelemetry::sdk::{
    export::trace::stdout,
    trace::{self, Sampler},
    Resource,
};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use pic_v1::{add_profile_pic, add_review_pic, get_profile_pic, get_review_pic, remove_review_pic};
use ping_routes::ping;
use reply_v1::{add_reply, get_replies, remove_reply};
use reqwest::ClientBuilder;
use review_v1::{
    add_review, edit_review, get_latest, get_reviews_from_loc, get_reviews_from_map_bounds,
    remove_review,
};
use sqlx::MySqlPool;
use std::{collections::HashMap, env, time::Duration};
use user_v1::{get_user_by_id, get_user_by_name, search_user_by_name, update_user};

mod auth;
mod authorization;
mod db;
mod friend_v1;
mod likes_v1;
mod pic_v1;
mod ping_routes;
mod reply_v1;
mod review_v1;
mod user_v1;

#[derive(Clone)]
pub struct Config {
    twilio_key: String,
    db_connection_string: String,
    signing_keys: SigningKeys,
    spaces_key: String,
    spaces_secret: String,
    newrelic_key: String,
}

const PIC_CONFIG_LIMIT: usize = 2_262_144;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Config = build_config();

    let http_client = ClientBuilder::new()
        .timeout(Duration::new(5, 0))
        .build()
        .unwrap();

    let pool: MySqlPool = MySqlPool::connect(&config.db_connection_string)
        .await
        .unwrap();

    let client = create_s3_client(&config.spaces_key, &config.spaces_secret);

    setup_tracing(&config);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(client.clone()))
            .app_data(Data::new(http_client.clone()))
            .wrap(Authorization)
            .service(web::scope("/ping").service(ping))
            .service(web::scope("/auth").service(request_code).service(sign_in))
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .service(
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
                        )
                        .service(
                            web::scope("/pic")
                                .service(get_profile_pic)
                                .service(add_profile_pic),
                        )
                        .service(
                            web::scope("/review")
                                .service(get_latest)
                                .service(get_review_pic)
                                .service(add_review_pic)
                                .service(remove_review_pic)
                                .service(get_reviews_from_map_bounds)
                                .service(get_reviews_from_loc)
                                .service(add_review)
                                .service(remove_review)
                                .service(edit_review),
                        )
                        .service(
                            web::scope("/user")
                                .service(search_user_by_name)
                                .service(get_user_by_id)
                                .service(get_user_by_name)
                                .service(update_user),
                        )
                        .service(
                            web::scope("/like")
                                .service(get_likes)
                                .service(like_review)
                                .service(unlike_review),
                        )
                        .service(
                            web::scope("/reply")
                                .service(get_replies)
                                .service(add_reply)
                                .service(remove_reply),
                        )
                        .app_data(PayloadConfig::new(PIC_CONFIG_LIMIT)),
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
            db_connection_string: String::from("mysql://root:test123@localhost:49581/mob"),
            signing_keys: encode_jwt_secret("thisisatestkey"),
            spaces_key: env::var("MOB_SPACES_KEY").unwrap(),
            spaces_secret: env::var("MOB_SPACES_SECRET").unwrap(),
            newrelic_key: String::from("Default"),
        },
        Err(_) => Config {
            twilio_key: env::var("TWILIO").unwrap(),
            db_connection_string: env::var("DB_CONNECTION").unwrap(),
            signing_keys: encode_jwt_secret(&env::var("JWT_KEY").unwrap()),
            spaces_key: env::var("SPACES_KEY").unwrap(),
            spaces_secret: env::var("SPACES_SECRET").unwrap(),
            newrelic_key: env::var("NR_KEY").unwrap(),
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

fn setup_tracing(config: &Config) {
    if config.newrelic_key != String::from("Default") {
        let mut nr_otlp_metadata: HashMap<String, String> = HashMap::new();
        nr_otlp_metadata.insert("api-key".to_string(), config.newrelic_key.clone());

        opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::HttpExporterBuilder::default()
                    .with_endpoint("https://otlp.nr-data.net/v1/traces")
                    .with_headers(nr_otlp_metadata),
            )
            .with_trace_config(
                trace::config()
                    .with_sampler(Sampler::AlwaysOn)
                    .with_resource(Resource::new(vec![KeyValue::new(
                        "service.name",
                        "bout-backend",
                    )])),
            )
            .install_batch(opentelemetry::runtime::Tokio)
            .unwrap();
    } else {
        stdout::new_pipeline()
            .with_trace_config(trace::config().with_sampler(Sampler::AlwaysOn))
            .install_simple();
    }
}
