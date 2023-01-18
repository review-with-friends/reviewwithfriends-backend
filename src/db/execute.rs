use images::DEFAULT_PIC_ID;
use sqlx::{types::chrono::Utc, Error, MySqlPool};
use uuid::Uuid;

use super::{Friend, Pic, Review, User};

/// Creates a user from the passed User struct.
/// Sets the pic_id to `DEFAULT_PIC_ID`
/// Does not generate a guid for `user.id`
/// Does not set a date for `user.created`
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

/// Creates a phoneauth record for tracking and validating user auth attempts.
/// ## Sets the `phoneauth.created` to `Utc::now().naive_utc()`
/// ## Sets the `phoneauth.id` to `Uuid::new_v4().to_string()`
/// ## Sets the `phoneauth.used` to `false`
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

/// Sets an `phoneauth.used` to `true` signalling it can no longer be used to generate a token.
pub async fn update_authattempt_used(client: &MySqlPool, id: &str) -> Result<(), Error> {
    sqlx::query("UPDATE phoneauth SET used = TRUE WHERE id = ?")
        .bind(id)
        .execute(client)
        .await?;

    return Ok(());
}

/// Creates an authattempt record for tracking attempts to auth as a user.
/// ## Sets the `authattempt.created` to `Utc::now().naive_utc()`
/// ## Sets the `authattempt.id` to `Uuid::new_v4().to_string()`
pub async fn create_authattempt(client: &MySqlPool, phone: &str) -> Result<(), Error> {
    sqlx::query("INSERT INTO authattempt (id, phone, created) VALUES (?,?,?)")
        .bind(Uuid::new_v4().to_string())
        .bind(&phone)
        .bind(Utc::now().naive_utc())
        .execute(client)
        .await?;

    return Ok(());
}

/// Creates a friend request as a user to another user.
/// ## Sets the `friendrequest.created` to `Utc::now().naive_utc()`
/// ## Sets the `friendrequest.id` to `Uuid::new_v4().to_string()`
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

/// Sets an incoming friend request for a user to ignored.
/// ## Sets the `friendrequest.ignored` to `true`
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

/// Deletes an incoming friend request for a user.
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

/// Deletes an outgoing friend request.
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

/// Accepts an incoming friend request from another user.
/// ## Transaction based.
/// Deletes both directions that could exist.
/// Creates both directions of the friend relationship.
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

/// Removes a friend.
/// ## Transaction based.
/// Attempts to remove both directiong of the friendship.
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

/// Creates a pic record in the database to be related to a user or review.
/// ## Sets the `pic.created` to `Utc::now().naive_utc()`
/// ## Sets the `pic.id` to `Uuid::new_v4().to_string()`
/// ## Sets the `pic.pic_handler` to `1` for now.
pub async fn create_pic(
    client: &MySqlPool,
    review_id: Option<String>,
    width: u16,
    height: u16,
) -> Result<Pic, Error> {
    let pic = Pic {
        id: Uuid::new_v4().to_string(),
        review_id: review_id,
        created: Utc::now().naive_utc(),
        pic_handler: 1,
        width: width,
        height: height,
    };

    sqlx::query(
        "INSERT INTO pic (id, created, pic_handler, review_id, width, height) VALUES (?,?,?,?,?,?)",
    )
    .bind(&pic.id)
    .bind(&pic.created)
    .bind(&pic.pic_handler)
    .bind(&pic.review_id)
    .bind(&pic.width)
    .bind(&pic.height)
    .execute(client)
    .await?;

    return Ok(pic);
}

/// Updates a `user.pic_id` for a given user.
/// The pic_id has no Foreign Key Constraints to the pic table.
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

/// Deletes a pic record.
/// Has no foreign key restrictions or harsh bindings to actual pic storage.
pub async fn delete_pic(client: &MySqlPool, pic_id: &str) -> Result<(), Error> {
    sqlx::query("DELETE FROM pic WHERE id = ?")
        .bind(pic_id)
        .execute(client)
        .await?;

    return Ok(());
}

/// Creates a review from the given `Review`.
/// Requires all fields to be set on incoming review. Validates nothing explicitly other than column constraints.
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

/// Updates a `review.pic_id`.
/// Does no validationg the pic exists.
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

/// Sets a `review.pic_id` to NULL.
pub async fn remove_review_pic_id(
    client: &MySqlPool,
    pic_id: &str,
    review_id: &str,
) -> Result<(), Error> {
    sqlx::query("DELETE FROM pic where id = ? and review_id = ?")
        .bind(pic_id)
        .bind(review_id)
        .execute(client)
        .await?;

    return Ok(());
}

/// Creates a like record associated with a given review.
/// ## Sets the `likes.created` to `Utc::now().naive_utc()`
/// ## Sets the `likes.id` to `Uuid::new_v4().to_string()`
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

/// Removes a like record.
pub async fn remove_like(client: &MySqlPool, user_id: &str, review_id: &str) -> Result<(), Error> {
    sqlx::query("DELETE FROM likes where user_id = ? and review_id = ?")
        .bind(user_id)
        .bind(review_id)
        .execute(client)
        .await?;

    return Ok(());
}

/// Removes a review and all likes/replies.
/// ## Transaction Based
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

/// Creates a reply record against a given review.
/// ## Sets the `reply.created` to `Utc::now().naive_utc()`
/// ## Sets the `reply.id` to `Uuid::new_v4().to_string()`
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

/// Deletes a given reply by id.
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

/// Updates a review by setting the passed values.
/// Will always set the passed values.
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

/// Updates a users names.
/// Will always try to set the passed values.
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
