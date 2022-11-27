use images::DEFAULT_PIC_ID;
use sqlx::{types::chrono::Utc, Error, MySqlPool};
use uuid::Uuid;

use super::{Friend, Pic, Review, User};

pub async fn create_user(client: &MySqlPool, user: &User) -> Result<(), Error> {
    sqlx::query(
        "INSERT INTO user (id, name, display_name, phone, created, pic_id) VALUES (?,?,?,?,?,?)",
    )
    .bind(&user.id)
    .bind(&user.name)
    .bind(&user.display_name)
    .bind(&user.phone)
    .bind(&user.created)
    .bind(DEFAULT_PIC_ID)
    .execute(client)
    .await?;

    return Ok(());
}

pub async fn create_phoneauth(client: &MySqlPool, phone: &str, code: &str) -> Result<(), Error> {
    sqlx::query("INSERT INTO phoneauth (id, phone, created, ip, code, used) VALUES (?,?,?,?,?,?)")
        .bind(Uuid::new_v4().to_string())
        .bind(&phone)
        .bind(Utc::now().naive_utc())
        .bind("")
        .bind(code)
        .bind(false)
        .execute(client)
        .await?;

    return Ok(());
}

pub async fn update_authattempt_used(client: &MySqlPool, id: &str) -> Result<(), Error> {
    sqlx::query("UPDATE phoneauth SET used = TRUE WHERE id = ?")
        .bind(id)
        .execute(client)
        .await?;

    return Ok(());
}

pub async fn create_authattempt(client: &MySqlPool, phone: &str) -> Result<(), Error> {
    sqlx::query("INSERT INTO authattempt (id, phone, created) VALUES (?,?,?)")
        .bind(Uuid::new_v4().to_string())
        .bind(&phone)
        .bind(Utc::now().naive_utc())
        .execute(client)
        .await?;

    return Ok(());
}

pub async fn create_friend_request(
    client: &MySqlPool,
    user_id: &str,
    friend_id: &str,
) -> Result<(), Error> {
    sqlx::query("INSERT INTO friendrequest (id, created, user_id, friend_id, ignored) VALUES (?,?,?,?,FALSE)")
        .bind(Uuid::new_v4().to_string())
        .bind(Utc::now().naive_utc())
        .bind(user_id)
        .bind(friend_id)
        .execute(client)
        .await?;

    return Ok(());
}

pub async fn ignore_friend_request(
    client: &MySqlPool,
    request_id: &str,
    friend_id: &str,
) -> Result<(), Error> {
    sqlx::query("UPDATE friendrequest SET ignored = true WHERE id = ? and friend_id = ?")
        .bind(request_id)
        .bind(friend_id)
        .execute(client)
        .await?;

    return Ok(());
}

pub async fn decline_friend_request(
    client: &MySqlPool,
    request_id: &str,
    friend_id: &str,
) -> Result<(), Error> {
    sqlx::query("DELETE FROM friendrequest WHERE id = ? and friend_id = ?")
        .bind(request_id)
        .bind(friend_id)
        .execute(client)
        .await?;

    return Ok(());
}

pub async fn cancel_friend_request(
    client: &MySqlPool,
    request_id: &str,
    user_id: &str,
) -> Result<(), Error> {
    sqlx::query("DELETE FROM friendrequest WHERE id = ? and user_id = ?")
        .bind(request_id)
        .bind(user_id)
        .execute(client)
        .await?;

    return Ok(());
}

pub async fn accept_friend_request(
    client: &MySqlPool,
    user_id: &str,
    friend_id: &str,
) -> Result<(), Error> {
    let mut trans = client.begin().await?; // encapsulate multiple actions into single transaction

    sqlx::query("DELETE FROM friendrequest WHERE user_id = ? and friend_id = ?")
        .bind(user_id)
        .bind(friend_id)
        .execute(&mut trans)
        .await?;

    sqlx::query("DELETE FROM friendrequest WHERE user_id = ? and friend_id = ?")
        .bind(friend_id)
        .bind(user_id)
        .execute(&mut trans)
        .await?;

    let user_friends_rows = sqlx::query("SELECT * FROM friend where user_id = ?")
        .bind(user_id)
        .fetch_all(&mut trans)
        .await?;

    let user_friends: Vec<Friend> = user_friends_rows.iter().map(|row| row.into()).collect();

    if !user_friends
        .iter()
        .any(|uf| -> bool { return uf.friend_id == friend_id })
    {
        sqlx::query("INSERT INTO friend (id, created, user_id, friend_id) VALUES (?,?,?,?)")
            .bind(Uuid::new_v4().to_string())
            .bind(Utc::now().naive_utc())
            .bind(user_id)
            .bind(friend_id)
            .execute(&mut trans)
            .await?;
    }

    let user_friends_rows = sqlx::query("SELECT * FROM friend where user_id = ?")
        .bind(friend_id)
        .fetch_all(&mut trans)
        .await?;

    let user_friends: Vec<Friend> = user_friends_rows.iter().map(|row| row.into()).collect();

    if !user_friends
        .iter()
        .any(|uf| -> bool { return uf.friend_id == user_id })
    {
        sqlx::query("INSERT INTO friend (id, created, user_id, friend_id) VALUES (?,?,?,?)")
            .bind(Uuid::new_v4().to_string())
            .bind(Utc::now().naive_utc())
            .bind(friend_id)
            .bind(user_id)
            .execute(&mut trans)
            .await?;
    }

    trans.commit().await?; // commit this bitch

    return Ok(());
}

pub async fn remove_current_friend(
    client: &MySqlPool,
    user_id: &str,
    friend_id: &str,
) -> Result<(), Error> {
    let mut trans = client.begin().await?;

    sqlx::query("DELETE FROM friend WHERE user_id = ? and friend_id = ?")
        .bind(user_id)
        .bind(friend_id)
        .execute(&mut trans)
        .await?;

    sqlx::query("DELETE FROM friend WHERE user_id = ? and friend_id = ?")
        .bind(friend_id)
        .bind(user_id)
        .execute(&mut trans)
        .await?;

    trans.commit().await?;

    return Ok(());
}

pub async fn create_pic(client: &MySqlPool) -> Result<Pic, Error> {
    let pic = Pic {
        id: Uuid::new_v4().to_string(),
        created: Utc::now().naive_utc(),
        pic_handler: 1,
    };

    sqlx::query("INSERT INTO pic (id, created, pic_handler) VALUES (?,?,?)")
        .bind(&pic.id)
        .bind(&pic.created)
        .bind(&pic.pic_handler)
        .execute(client)
        .await?;

    return Ok(pic);
}

pub async fn update_user_pic_id(
    client: &MySqlPool,
    pic_id: &str,
    user_id: &str,
) -> Result<(), Error> {
    sqlx::query("UPDATE user SET pic_id = ? WHERE id = ?")
        .bind(pic_id)
        .bind(user_id)
        .execute(client)
        .await?;

    return Ok(());
}

pub async fn delete_pic(client: &MySqlPool, pic_id: &str) -> Result<(), Error> {
    sqlx::query("DELETE FROM pic WHERE id = ?")
        .bind(pic_id)
        .execute(client)
        .await?;

    return Ok(());
}

pub async fn create_review(client: &MySqlPool, review: &Review) -> Result<(), Error> {
    sqlx::query(
        "INSERT INTO review 
        (id, user_id, created, pic_id, text, stars, location_name, latitude, longitude, is_custom, category) 
        VALUES (?,?,?,?,?,?,?,?,?,?,?)",
    )
    .bind(&review.id)
    .bind(&review.user_id)
    .bind(&review.created)
    .bind(&review.pic_id)
    .bind(&review.text)
    .bind(&review.stars)
    .bind(&review.location_name)
    .bind(&review.latitude)
    .bind(&review.longitude)
    .bind(&review.is_custom)
    .bind(&review.category)
    .execute(client)
    .await?;

    return Ok(());
}

pub async fn update_review_pic_id(
    client: &MySqlPool,
    pic_id: &str,
    review_id: &str,
) -> Result<(), Error> {
    sqlx::query("UPDATE review SET pic_id = ? WHERE id = ?")
        .bind(pic_id)
        .bind(review_id)
        .execute(client)
        .await?;

    return Ok(());
}

pub async fn remove_review_pic_id(client: &MySqlPool, review_id: &str) -> Result<(), Error> {
    const NULL: Option<String> = None;
    sqlx::query("UPDATE review SET pic_id = ? WHERE id = ?")
        .bind(NULL)
        .bind(review_id)
        .execute(client)
        .await?;

    return Ok(());
}

pub async fn create_like(client: &MySqlPool, user_id: &str, review_id: &str) -> Result<(), Error> {
    sqlx::query(
        "INSERT INTO likes 
        (id, created, user_id, review_id)
        VALUES (?,?,?,?)",
    )
    .bind(Uuid::new_v4().to_string())
    .bind(Utc::now().naive_utc())
    .bind(user_id)
    .bind(review_id)
    .execute(client)
    .await?;

    return Ok(());
}

pub async fn remove_like(client: &MySqlPool, user_id: &str, review_id: &str) -> Result<(), Error> {
    sqlx::query("DELETE FROM likes where user_id = ? and review_id = ?")
        .bind(user_id)
        .bind(review_id)
        .execute(client)
        .await?;

    return Ok(());
}

pub async fn remove_review_and_children(client: &MySqlPool, review_id: &str) -> Result<(), Error> {
    let mut trans = client.begin().await?;

    sqlx::query("DELETE FROM reply WHERE review_id = ?")
        .bind(review_id)
        .execute(&mut trans)
        .await?;

    sqlx::query("DELETE FROM likes WHERE review_id = ?")
        .bind(review_id)
        .execute(&mut trans)
        .await?;

    sqlx::query("DELETE FROM review WHERE id = ?")
        .bind(review_id)
        .execute(&mut trans)
        .await?;

    trans.commit().await?;

    return Ok(());
}

pub async fn create_reply(
    client: &MySqlPool,
    user_id: &str,
    review_id: &str,
    text: &str,
) -> Result<(), Error> {
    sqlx::query(
        "INSERT INTO reply 
        (id, created, user_id, review_id, text)
        VALUES (?,?,?,?,?)",
    )
    .bind(Uuid::new_v4().to_string())
    .bind(Utc::now().naive_utc())
    .bind(user_id)
    .bind(review_id)
    .bind(text)
    .execute(client)
    .await?;

    return Ok(());
}

pub async fn delete_reply(
    client: &MySqlPool,
    reply_id: &str,
    review_id: &str,
    user_id: &str,
) -> Result<(), Error> {
    sqlx::query("DELETE FROM reply WHERE id = ? AND review_id = ? and user_id = ?")
        .bind(reply_id)
        .bind(review_id)
        .bind(user_id)
        .execute(client)
        .await?;

    return Ok(());
}

pub async fn update_review(
    client: &MySqlPool,
    review_id: &str,
    stars: u8,
    text: &str,
) -> Result<(), Error> {
    sqlx::query("UPDATE review SET stars = ?, text = ? WHERE id = ?")
        .bind(stars)
        .bind(text)
        .bind(review_id)
        .execute(client)
        .await?;

    return Ok(());
}

pub async fn update_usernames(
    client: &MySqlPool,
    user_id: &str,
    display_name: &str,
    name: &str,
) -> Result<(), Error> {
    sqlx::query("UPDATE user SET display_name = ?, name = ? WHERE id = ?")
        .bind(display_name)
        .bind(name)
        .bind(user_id)
        .execute(client)
        .await?;

    return Ok(());
}
