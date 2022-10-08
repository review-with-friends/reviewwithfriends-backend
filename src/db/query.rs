use rocket_db_pools::{
    sqlx::{self, Error, Row},
    Connection, Database,
};

use super::User;

#[derive(Database)]
#[database("mob")]
pub struct DBClient(sqlx::MySqlPool);

pub async fn get_user(mut client: Connection<DBClient>, id: String) -> Result<User, Error> {
    let row = sqlx::query("SELECT * FROM user where id = ?")
        .bind(id)
        .fetch_one(&mut *client)
        .await?;

    return Ok(row.into());
}
