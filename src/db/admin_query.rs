use sqlx::{Error, MySqlPool, Row};

/// Gets the total number of active users.
pub async fn get_total_user_count(client: &MySqlPool) -> Result<i64, Error> {
    let row = sqlx::query("SELECT count(*) FROM user")
        .fetch_one(client)
        .await?;

    return Ok(row.get(0));
}
