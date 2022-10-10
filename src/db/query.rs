use chrono::Duration;
use rocket_db_pools::sqlx::{self, Error};
use sqlx::types::chrono::Utc;

use super::{AuthAttempt, DBClient, PhoneAuth, User};

pub async fn get_user(client: &DBClient, id: String) -> Result<User, Error> {
    let row = sqlx::query("SELECT * FROM user where id = ?")
        .bind(id)
        .fetch_one(&client.0)
        .await?;

    return Ok((&row).into());
}

pub async fn get_user_by_phone(client: &DBClient, phone: String) -> Result<Option<User>, Error> {
    let rows = sqlx::query("SELECT * FROM user where phone = ?")
        .bind(phone)
        .fetch_all(&client.0)
        .await?;

    if rows.len() != 1 {
        return Ok(None);
    }

    return Ok(Some((&(rows.first())).unwrap().into()));
}

pub async fn get_current_phoneauths(
    client: &DBClient,
    phone: String,
) -> Result<Vec<PhoneAuth>, Error> {
    let rows =
        sqlx::query("SELECT * FROM phoneauth where created > ? and phone = ? and used = FALSE")
            .bind(Utc::now().naive_utc() - Duration::hours(1))
            .bind(phone)
            .fetch_all(&client.0)
            .await?;

    let out: Vec<PhoneAuth> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

pub async fn get_phoneauth_attempts(
    client: &DBClient,
    phone: String,
) -> Result<Vec<AuthAttempt>, Error> {
    let rows = sqlx::query("SELECT * FROM authattempt where created > ? and phone = ?")
        .bind(Utc::now().naive_utc() - Duration::hours(1))
        .bind(phone)
        .fetch_all(&client.0)
        .await?;

    let out: Vec<AuthAttempt> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}
