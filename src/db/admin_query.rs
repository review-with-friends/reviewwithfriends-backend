use sqlx::{Error, MySqlPool, Row};

use crate::db::Report;

/// Gets the total number of active users.
pub async fn get_total_user_count(client: &MySqlPool) -> Result<i64, Error> {
    let row = sqlx::query("SELECT count(*) FROM user")
        .fetch_one(client)
        .await?;

    return Ok(row.get(0));
}

/// Simply gets a ping record.
pub async fn get_all_reports(client: &MySqlPool) -> Result<Vec<Report>, Error> {
    let reports = sqlx::query_as!(
        Report,
        "SELECT *
        FROM reports"
    )
    .fetch_all(client)
    .await?;

    return Ok(reports);
}
