use crate::{
    db::{get_user, query::DBClient},
    JWTAuthorized,
};
use rocket::{http::Status, response::status};
use rocket_db_pools::{
    sqlx::{self, Row},
    Connection,
};

#[get("/helloworld")]
pub async fn hello_world(mut client: Connection<DBClient>) -> status::Custom<String> {
    let user_res = get_user(client, "efd0b883-5671-420e-94dc-a18ae65384d3".to_string()).await;

    match user_res {
        Ok(user) => {
            return status::Custom(Status::Ok, user.id);
        }
        Err(err) => status::Custom(Status::Ok, err.to_string()),
    }
}

#[get("/auth_helloworld")]
pub async fn auth_hello_world(
    _auth: JWTAuthorized,
    mut client: Connection<DBClient>,
) -> status::Custom<String> {
    let query_resp: Option<String> = sqlx::query("SELECT * FROM users")
        .fetch_one(&mut *client)
        .await
        .and_then(|r| Ok(r.try_get("name")?))
        .ok();

    match query_resp {
        Some(resp) => {
            return status::Custom(Status::Ok, String::from(resp));
        }
        None => status::Custom(Status::Ok, String::from("Everything is BAD!")),
    }
}
