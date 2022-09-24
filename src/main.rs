#[macro_use]
extern crate rocket;

use std::error::Error;

use rocket::{http::Status, response::status};
use rocket_db_pools::{
    sqlx::{self, Row},
    Connection, Database,
};

#[derive(Database)]
#[database("mob")]
struct DBClient(sqlx::MySqlPool);

#[launch]
async fn rocket() -> _ {
    println!("Mob has started.");

    rocket::build()
        .attach(DBClient::init())
        .mount("/api/test", routes![hello_world])
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
