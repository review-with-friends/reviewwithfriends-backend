use crate::user_v1::update_user_device_token;
use actix_cors::Cors;
use actix_web::{
    http,
    web::{self, Data, PayloadConfig},
    App, HttpServer,
};
use actix_web_opentelemetry::RequestTracing;
use admin_v1::{get_all_reports, get_user_count};
use auth::*;
use authorization::Authentication;
use chrono::Utc;
use friend_v1::{
    accept_friend, add_friend, cancel_friend, decline_friend, discover_friends, full_friends,
    get_friends, get_ignored_friends, get_incoming_friends, get_outgoing_friends,
    get_user_friends::get_user_friends, ignore_friend, remove_friend,
};
use images::create_s3_client;
use jwt::{encode_apn_jwt_secret, encode_jwt_secret, mint_apn_jwt, APNSigningKey, SigningKeys};
use likes_v1::{
    get_current_liked_reviews_full, get_current_likes, get_likes, like_review, unlike_review,
};
use moka::sync::Cache;
use notifications_v1::{
    confirm_notifications, get_notifications, start_notification_worker, APNClient,
    NotificationQueue,
};
use opentelemetry::{
    sdk::{
        export::trace::stdout,
        trace::{self, Sampler},
        Resource,
    },
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use pic_v1::{add_profile_pic, add_review_pic, get_profile_pic, remove_review_pic};
use ping_routes::ping;
use ratelimit::RateLimit;
use reply_v1::{add_reply, get_replies, remove_reply};
use report_v1::{report_bug, report_user, GithubClient};
use reqwest::ClientBuilder;
use review_v1::{
    add_review, edit_review, get_full_reviews_from_user, get_latest, get_latest_full,
    get_review_by_id, get_reviews_from_loc, get_reviews_from_map_bounds,
    get_reviews_from_map_bounds_with_exclusions, get_reviews_from_user, remove_review,
    search_latest,
};
use sqlx::MySqlPool;
use std::sync::Mutex;
use std::{collections::HashMap, env, time::Duration};
use user_v1::{
    get_me, get_user_by_id, get_user_by_name, search_user_by_name, update_user,
    update_user_recovery_email,
};

mod admin_v1;
mod auth;
mod authorization;
mod compound_types;
mod db;
mod friend_v1;
mod likes_v1;
mod notifications_v1;
mod pic_v1;
mod ping_routes;
mod ratelimit;
mod reply_v1;
mod report_v1;
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
    apn_key: APNSigningKey,
    sendgrid_key: String,
    github_key: String,
}

const PIC_CONFIG_LIMIT: usize = 4_262_144;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Config = build_config();

    let http_client = ClientBuilder::new()
        .timeout(Duration::new(5, 0))
        .build()
        .unwrap();

    let apn_token = mint_apn_jwt(&config.apn_key);

    let apn_client = web::Data::new(APNClient {
        client: ClientBuilder::new()
            .http2_prior_knowledge()
            .timeout(Duration::new(5, 0))
            .build()
            .unwrap(),
        key: config.apn_key.clone(),
        token: Mutex::new(apn_token),
        issued_time: Mutex::new(Utc::now().timestamp()),
    });

    let gh_client = web::Data::new(GithubClient {
        client: ClientBuilder::new()
            .timeout(Duration::new(5, 0))
            .build()
            .unwrap(),
        token: config.github_key.clone(),
    });

    let pool: MySqlPool = MySqlPool::connect(&config.db_connection_string)
        .await
        .unwrap();

    let client = create_s3_client(&config.spaces_key, &config.spaces_secret);

    setup_tracing(&config);

    let queue = Data::new(Mutex::new(NotificationQueue::new()));

    start_notification_worker(queue.clone(), apn_client.clone(), Data::new(pool.clone()));

    let ratelimit_cache = setup_moka_cache();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(ratelimit_cache.clone()))
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(client.clone()))
            .app_data(Data::new(http_client.clone()))
            .app_data(apn_client.clone())
            .app_data(gh_client.clone())
            .app_data(queue.clone())
            .app_data(PayloadConfig::new(PIC_CONFIG_LIMIT))
            .wrap(RateLimit)
            .wrap(Authentication)
            .wrap(RequestTracing::new())
            .wrap(
                Cors::default()
                    .allowed_origin("https://reviewwithfriends.com")
                    .allowed_origin("http://localhost:8000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(web::scope("/ping").service(ping))
            .service(
                web::scope("/auth")
                    .service(request_code)
                    .service(sign_in)
                    .service(sign_in_demo)
                    .service(recovery_code)
                    .service(update_phone),
            )
            .service(
                web::scope("/admin")
                    .service(get_user_count)
                    .service(get_all_reports),
            )
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .service(
                            web::scope("/friends")
                                .service(full_friends)
                                .service(get_friends)
                                .service(get_outgoing_friends)
                                .service(get_incoming_friends)
                                .service(get_ignored_friends)
                                .service(add_friend)
                                .service(accept_friend)
                                .service(cancel_friend)
                                .service(decline_friend)
                                .service(remove_friend)
                                .service(ignore_friend)
                                .service(discover_friends)
                                .service(get_user_friends),
                        )
                        .service(
                            web::scope("/pic")
                                .service(get_profile_pic)
                                .service(add_profile_pic),
                        )
                        .service(
                            web::scope("/review")
                                .service(get_latest)
                                .service(get_latest_full)
                                .service(add_review_pic)
                                .service(remove_review_pic)
                                .service(get_reviews_from_map_bounds)
                                .service(get_reviews_from_map_bounds_with_exclusions)
                                .service(get_reviews_from_loc)
                                .service(get_reviews_from_user)
                                .service(get_full_reviews_from_user)
                                .service(add_review)
                                .service(remove_review)
                                .service(get_review_by_id)
                                .service(search_latest)
                                .service(edit_review),
                        )
                        .service(
                            web::scope("/user")
                                .service(search_user_by_name)
                                .service(get_user_by_id)
                                .service(get_user_by_name)
                                .service(update_user)
                                .service(get_me)
                                .service(update_user_device_token)
                                .service(update_user_recovery_email),
                        )
                        .service(
                            web::scope("/like")
                                .service(get_likes)
                                .service(like_review)
                                .service(unlike_review)
                                .service(get_current_likes)
                                .service(get_current_liked_reviews_full),
                        )
                        .service(
                            web::scope("/reply")
                                .service(get_replies)
                                .service(add_reply)
                                .service(remove_reply),
                        )
                        .service(
                            web::scope("/notification")
                                .service(get_notifications)
                                .service(confirm_notifications),
                        )
                        .service(
                            web::scope("/report")
                                .service(report_user)
                                .service(report_bug),
                        ),
                ),
            )
    })
    .bind(build_binding())?
    .run()
    .await
}

/// Fetch build configs from environment variable
fn build_config() -> Config {
    let is_dev = env::var("MOB_DEV");

    match is_dev {
        Ok(_) => Config {
            twilio_key: String::from("123"),
            db_connection_string: env::var("DATABASE_URL").unwrap(),
            signing_keys: encode_jwt_secret("thisisatestkey"),
            spaces_key: env::var("MOB_SPACES_KEY").unwrap(),
            spaces_secret: env::var("MOB_SPACES_SECRET").unwrap(),
            newrelic_key: String::from("Default"),
            apn_key: encode_apn_jwt_secret(&env::var("APN_KEY").unwrap()),
            sendgrid_key: env::var("SENDGRID_KEY").unwrap(),
            github_key: env::var("GITHUB_KEY").unwrap(),
        },
        Err(_) => Config {
            twilio_key: env::var("TWILIO").unwrap(),
            db_connection_string: env::var("DB_CONNECTION").unwrap(),
            signing_keys: encode_jwt_secret(&env::var("JWT_KEY").unwrap()),
            spaces_key: env::var("SPACES_KEY").unwrap(),
            spaces_secret: env::var("SPACES_SECRET").unwrap(),
            newrelic_key: env::var("NR_KEY").unwrap(),
            apn_key: encode_apn_jwt_secret(&env::var("APN_KEY").unwrap()),
            sendgrid_key: env::var("SENDGRID_KEY").unwrap(),
            github_key: env::var("GITHUB_KEY").unwrap(),
        },
    }
}

/// Set port bindings
fn build_binding() -> (&'static str, u16) {
    let is_dev = env::var("MOB_DEV");

    match is_dev {
        Ok(_) => ("127.0.0.1", 8081),
        Err(_) => ("0.0.0.0", 80),
    }
}

/// Initialize tracing - New Relic for Prod, stdout for dev
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

fn setup_moka_cache() -> Cache<String, usize> {
    Cache::builder()
        .time_to_live(Duration::from_secs(60))
        .max_capacity(10000)
        .build()
}
