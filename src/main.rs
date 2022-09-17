#[macro_use]
extern crate rocket;

use std::env;

use rocket::{http::Status, response::status};
use rocket_db_pools::{mongodb, Connection, Database};

#[derive(Database)]
#[database("mob")]
struct MongoDBClient(mongodb::Client);

#[launch]
async fn rocket() -> _ {
    println!("wow");
    let name = "ROCKET_DATABASES";
    match env::var(name) {
        Ok(v) => println!("{}: {}", name, v),
        Err(e) => panic!("${} is not set ({})", name, e),
    }

    rocket::build()
        .attach(MongoDBClient::init())
        .mount("/api/test", routes![hello_world])
}

#[get("/helloworld")]
async fn hello_world(client: Connection<MongoDBClient>) -> status::Custom<String> {
    let opt_dbs = client.list_database_names(None, None).await;
    match opt_dbs {
        Ok(dbs) => {
            for db_name in dbs {
                println!("{}", db_name);
            }
            status::Custom(Status::Ok, String::from("Everything is OK!"))
        }
        Err(err) => panic!("Problem opening the file: {:?}", err),
    }
}
