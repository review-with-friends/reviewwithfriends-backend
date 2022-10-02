#[macro_use]
extern crate rocket;

use ::serde::Deserialize;
use reqwest::ClientBuilder;
use rocket::{fairing::AdHoc, http::Status, response::status, State};
use rocket_db_pools::{
    sqlx::{self, Row},
    Connection, Database,
};
use std::{collections::HashMap, time::Duration};
use urlencoding::encode;

#[derive(Database)]
#[database("mob")]
struct DBClient(sqlx::MySqlPool);

#[derive(Deserialize)]
struct Config {
    twilio: String,
}

#[launch]
async fn rocket() -> _ {
    println!("Mob has started.");

    rocket::build()
        .attach(DBClient::init())
        .attach(AdHoc::config::<Config>())
        .mount("/api/test", routes![hello_world, phone_auth])
}

#[get("/helloworld")]
async fn hello_world(mut client: Connection<DBClient>) -> status::Custom<String> {
    let query_resp: Option<String> = sqlx::query("SELECT * FROM users")
        .fetch_one(&mut *client)
        .await
        .and_then(|r| Ok(r.try_get(2)?))
        .ok();

    match query_resp {
        Some(resp) => {
            return status::Custom(Status::Ok, String::from(resp));
        }
        None => status::Custom(Status::Ok, String::from("Everything is BAD!")),
    }
}

#[get("/phoneauth")]
async fn phone_auth(
    mut client: Connection<DBClient>,
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
        //.body(encode("Body=Hello from Twilio\n\rFrom=+17246134841\n\rTo=+17014910059").into_owned())
        .basic_auth("AC0094c61aa39fc9c673130f6e28e43bad", Some(&config.twilio))
        .send()
        .await
        .unwrap();
    return status::Custom(Status::Ok, String::from(response.text().await.unwrap()));
}
