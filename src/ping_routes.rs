use rocket::{http::Status, response::status::Custom};

use crate::db::{get_ping, DBClient};

const PING_ID: &str = "123";

#[get("/")]
pub async fn ping(client: &DBClient) -> Result<String, Custom<String>> {
    let ping_res = get_ping(client, PING_ID).await;

    match ping_res {
        Ok(ping) => Ok(ping),
        Err(_) => {
            return Err(Custom(
                Status::InternalServerError,
                "could not get ping".to_string(),
            ))
        }
    }
}
