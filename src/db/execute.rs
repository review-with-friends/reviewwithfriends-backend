use chrono::Duration;
use rand::Rng;
use rocket_db_pools::{
    sqlx::{self, Error, Row},
    Connection, Database,
};
use sqlx::types::chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

use super::{AuthAttempt, DBClient, PhoneAuth, User};

pub async fn create_user(client: &DBClient, user: &User) -> Result<(), Error> {
    sqlx::query("INSERT INTO user (id, name, display_name, phone, created) VALUES (?,?,?,?,?)")
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.display_name)
        .bind(&user.phone)
        .bind(&user.created)
        .execute(&client.0)
        .await?;

    return Ok(());
}

pub async fn create_phoneauth(client: &DBClient, phone: &str, code: &str) -> Result<(), Error> {
    sqlx::query("INSERT INTO phoneauth (id, phone, created, ip, code, used) VALUES (?,?,?,?,?,?)")
        .bind(Uuid::new_v4().to_string())
        .bind(&phone)
        .bind(Utc::now().naive_utc())
        .bind("")
        .bind(code)
        .bind(false)
        .execute(&client.0)
        .await?;

    return Ok(());
}

pub async fn update_authattempt_used(client: &DBClient, id: &str) -> Result<(), Error> {
    sqlx::query("UPDATE phoneauth SET used = TRUE WHERE id = ?")
        .bind(id)
        .execute(&client.0)
        .await?;

    return Ok(());
}

pub async fn create_authattempt(client: &DBClient, phone: &str, code: &str) -> Result<(), Error> {
    sqlx::query("INSERT INTO authattempt (id, phone, created) VALUES (?,?,?)")
        .bind(Uuid::new_v4().to_string())
        .bind(&phone)
        .bind(Utc::now().naive_utc())
        .execute(&client.0)
        .await?;

    return Ok(());
}
