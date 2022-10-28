use chrono::Duration;
use rocket_db_pools::sqlx::{self, Error};
use sqlx::{types::chrono::Utc, Row};

use super::{AuthAttempt, DBClient, Friend, FriendRequest, PhoneAuth, User};

pub async fn get_ping(client: &DBClient, id: &str) -> Result<String, Error> {
    let row = sqlx::query("SELECT * FROM ping where id = ?")
        .bind(id)
        .fetch_one(&client.0)
        .await?;

    return Ok(row.try_get("id")?);
}

pub async fn get_user(client: &DBClient, id: String) -> Result<User, Error> {
    let row = sqlx::query("SELECT * FROM user where id = ?")
        .bind(id)
        .fetch_one(&client.0)
        .await?;

    return Ok((&row).into());
}

pub async fn does_user_exist(client: &DBClient, id: &str) -> Result<bool, Error> {
    let row = sqlx::query("SELECT * FROM user where id = ?")
        .bind(id)
        .fetch_all(&client.0)
        .await?;

    return Ok(row.len() == 1);
}

pub async fn get_user_by_phone(client: &DBClient, phone: &str) -> Result<Option<User>, Error> {
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

pub async fn get_incoming_friend_requests(
    client: &DBClient,
    user_id: String,
) -> Result<Vec<FriendRequest>, Error> {
    let rows = sqlx::query("SELECT * FROM friendrequest where friend_id = ? and ignored = false")
        .bind(user_id)
        .fetch_all(&client.0)
        .await?;

    let out: Vec<FriendRequest> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

pub async fn get_incoming_ignored_friend_requests(
    client: &DBClient,
    user_id: String,
) -> Result<Vec<FriendRequest>, Error> {
    let rows = sqlx::query("SELECT * FROM friendrequest where friend_id = ? and ignored = true")
        .bind(user_id)
        .fetch_all(&client.0)
        .await?;

    let out: Vec<FriendRequest> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

pub async fn get_outgoing_friend_requests(
    client: &DBClient,
    user_id: String,
) -> Result<Vec<FriendRequest>, Error> {
    let rows = sqlx::query("SELECT * FROM friendrequest where user_id = ?")
        .bind(user_id)
        .fetch_all(&client.0)
        .await?;

    let out: Vec<FriendRequest> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

pub async fn get_current_friends(client: &DBClient, user_id: String) -> Result<Vec<Friend>, Error> {
    let rows = sqlx::query("SELECT * FROM friend where user_id = ?")
        .bind(user_id)
        .fetch_all(&client.0)
        .await?;

    let out: Vec<Friend> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}
