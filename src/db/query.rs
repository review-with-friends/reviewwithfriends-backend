use chrono::Duration;
use sqlx::{types::chrono::Utc, Error, MySqlPool, Row};

use super::{AuthAttempt, Friend, FriendRequest, Like, PhoneAuth, Pic, Reply, Review, User};

/// Simply gets a ping record.
pub async fn get_ping(client: &MySqlPool, id: &str) -> Result<String, Error> {
    let row = sqlx::query("SELECT * FROM ping WHERE id = ?")
        .bind(id)
        .fetch_one(client)
        .await?;

    return Ok(row.try_get("id")?);
}

/// Tries to get a user by `user.id`, and will return `None` if not found.
pub async fn get_user(client: &MySqlPool, id: &str) -> Result<Option<User>, Error> {
    let row_opt = sqlx::query("SELECT * FROM user WHERE id = ?")
        .bind(id)
        .fetch_optional(client)
        .await?;

    if let Some(row) = row_opt {
        return Ok(Some((&row).into()));
    } else {
        return Ok(None);
    }
}

/// Tries to get a user by exact `user.name`, and will return `None` if not found.
pub async fn get_user_from_name(client: &MySqlPool, name: &str) -> Result<Option<User>, Error> {
    let row_opt = sqlx::query("SELECT * FROM user WHERE name = ?")
        .bind(name)
        .fetch_optional(client)
        .await?;

    if let Some(row) = row_opt {
        return Ok(Some((&row).into()));
    } else {
        return Ok(None);
    }
}

/// Gets a list of users from the given `user.name`.
/// This search is a trailing wildcard and limits to top 50 results.
pub async fn search_user_from_name(client: &MySqlPool, name: &str) -> Result<Vec<User>, Error> {
    let search_term = format!("{}%", name.replace("%", ""));
    let rows = sqlx::query("SELECT * FROM user WHERE name LIKE ? LIMIT 50")
        .bind(search_term)
        .fetch_all(client)
        .await?;

    let out: Vec<User> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

/// Gets the existence of a user by `user.id`. True if exists, false if not.
pub async fn does_user_exist(client: &MySqlPool, id: &str) -> Result<bool, Error> {
    let row = sqlx::query("SELECT * FROM user WHERE id = ?")
        .bind(id)
        .fetch_all(client)
        .await?;

    return Ok(row.len() == 1);
}

/// Gets the existence of a user by `user.name`. True if exists, false if not.
pub async fn does_user_exist_by_name(client: &MySqlPool, name: &str) -> Result<bool, Error> {
    let row = sqlx::query("SELECT * FROM user WHERE name = ?")
        .bind(name)
        .fetch_all(client)
        .await?;

    return Ok(row.len() == 1);
}

/// Gets a given user by `user.phone`, and will return `None` if not found.
pub async fn get_user_by_phone(client: &MySqlPool, phone: &str) -> Result<Option<User>, Error> {
    let rows = sqlx::query("SELECT * FROM user WHERE phone = ?")
        .bind(phone)
        .fetch_all(client)
        .await?;

    if rows.len() != 1 {
        return Ok(None);
    }

    return Ok(Some((&(rows.first())).unwrap().into()));
}

/// Gets the current phoneauths.
/// Results are within the last 1 hour of `phoneauth.created`, and `phoneauth.used` is `false`
pub async fn get_current_phoneauths(
    client: &MySqlPool,
    phone: &str,
) -> Result<Vec<PhoneAuth>, Error> {
    let rows =
        sqlx::query("SELECT * FROM phoneauth WHERE created > ? and phone = ? and used = FALSE")
            .bind(Utc::now().naive_utc() - Duration::hours(1))
            .bind(phone)
            .fetch_all(client)
            .await?;

    let out: Vec<PhoneAuth> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

/// Gets the current authattempts for a given `authattempt.phone` in the last 1 hour of `authattempt.created`.
pub async fn get_phoneauth_attempts(
    client: &MySqlPool,
    phone: &str,
) -> Result<Vec<AuthAttempt>, Error> {
    let rows = sqlx::query("SELECT * FROM authattempt WHERE created > ? and phone = ?")
        .bind(Utc::now().naive_utc() - Duration::hours(1))
        .bind(phone)
        .fetch_all(client)
        .await?;

    let out: Vec<AuthAttempt> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

/// Gets the users current incoming friend requests that `friendrequest.ignored` is false.
pub async fn get_incoming_friend_requests(
    client: &MySqlPool,
    user_id: &str,
) -> Result<Vec<FriendRequest>, Error> {
    let rows = sqlx::query("SELECT * FROM friendrequest WHERE friend_id = ? and ignored = false")
        .bind(user_id)
        .fetch_all(client)
        .await?;

    let out: Vec<FriendRequest> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

/// Gets the users current incoming friend requests that `friendrequest.ignored` is true.
pub async fn get_incoming_ignored_friend_requests(
    client: &MySqlPool,
    user_id: &str,
) -> Result<Vec<FriendRequest>, Error> {
    let rows = sqlx::query("SELECT * FROM friendrequest WHERE friend_id = ? and ignored = true")
        .bind(user_id)
        .fetch_all(client)
        .await?;

    let out: Vec<FriendRequest> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

/// Gets the users outgoing friend requests.
pub async fn get_outgoing_friend_requests(
    client: &MySqlPool,
    user_id: &str,
) -> Result<Vec<FriendRequest>, Error> {
    let rows = sqlx::query("SELECT * FROM friendrequest WHERE user_id = ?")
        .bind(user_id)
        .fetch_all(client)
        .await?;

    let out: Vec<FriendRequest> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

/// Gets the users current friends list.
/// Not paged.
pub async fn get_current_friends(client: &MySqlPool, user_id: &str) -> Result<Vec<Friend>, Error> {
    let rows = sqlx::query("SELECT * FROM friend WHERE user_id = ?")
        .bind(user_id)
        .fetch_all(client)
        .await?;

    let out: Vec<Friend> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

/// Gets a pic record for a specific id, will return `None` if it doesn't exists.
pub async fn get_pic(client: &MySqlPool, id: &str) -> Result<Option<Pic>, Error> {
    let row_opt = sqlx::query("SELECT * FROM pic WHERE id = ?")
        .bind(id)
        .fetch_optional(client)
        .await?;

    if let Some(row) = row_opt {
        return Ok(Some((&row).into()));
    } else {
        return Ok(None);
    }
}

/// Gets a single review for the given review_id.
/// Accounts for the passed user_id's fiends and own reviews.
pub async fn get_review(
    client: &MySqlPool,
    user_id: &str,
    review_id: &str,
) -> Result<Option<Review>, Error> {
    let row_opt = sqlx::query(
        "SELECT r1.* FROM review as r1
    INNER JOIN friend as f1 ON f1.friend_id = r1.user_id 
    WHERE f1.user_id = ? AND r1.id = ?
    UNION
    SELECT r2.* from review as r2
    WHERE r2.user_id = ? AND r2.id = ?",
    )
    .bind(user_id)
    .bind(review_id)
    .bind(user_id)
    .bind(review_id)
    .fetch_optional(client)
    .await?;

    if let Some(row) = row_opt {
        return Ok(Some((&row).into()));
    } else {
        return Ok(None);
    }
}

/// Gets all reviews from a given name, latitude, and longitude combination.
/// Accounts for the passed user_id's fiends and own reviews.
/// ## Results are NOT paged.
pub async fn get_reviews_from_location(
    client: &MySqlPool,
    user_id: &str,
    name: &str,
    latitude_low: f64,
    latitude_high: f64,
    longitude_low: f64,
    longitude_high: f64,
) -> Result<Vec<Review>, Box<dyn std::error::Error>> {
    let rows = sqlx::query(
        "SELECT r1.* FROM review as r1
        INNER JOIN friend as f1 ON f1.friend_id = r1.user_id 
        WHERE f1.user_id = ? AND r1.location_name = ? AND r1.latitude BETWEEN ? AND ? AND r1.longitude BETWEEN ? AND ?
        UNION
        SELECT r2.* FROM review as r2
        WHERE r2.user_id = ? AND r2.location_name = ? AND r2.latitude BETWEEN ? AND ? AND r2.longitude BETWEEN ? AND ?
        ",
    )
    .bind(user_id)
    .bind(name)
    .bind(latitude_low)
    .bind(latitude_high)
    .bind(longitude_low)
    .bind(longitude_high)
    .bind(user_id)
    .bind(name)
    .bind(latitude_low)
    .bind(latitude_high)
    .bind(longitude_low)
    .bind(longitude_high)
    .fetch_all(client)
    .await?;

    let out: Vec<Review> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

/// Gets reviews from a given bounding box.
/// Accounts for the passed user_id's friends and own reviews.
/// ## Results are paged.
pub async fn get_reviews_from_bounds(
    client: &MySqlPool,
    user_id: &str,
    latitude_north: f64,
    latitude_south: f64,
    longitude_west: f64,
    longitude_east: f64,
    page: u32,
) -> Result<Vec<Review>, Box<dyn std::error::Error>> {
    const PAGE_SIZE: u32 = 500;
    let lower_count = page * PAGE_SIZE;
    let higher_count = lower_count + PAGE_SIZE;
    let rows = sqlx::query(
        "SELECT * FROM (SELECT r1.* FROM review as r1
        INNER JOIN friend as f1 ON f1.friend_id = r1.user_id
        WHERE f1.user_id = ? 
        AND r1.latitude <= ? AND r1.latitude >= ? AND r1.longitude >= ? AND r1.longitude <= ?
        UNION ALL
        SELECT r2.* FROM review as r2
        WHERE r2.user_id = ? 
        AND r2.latitude <= ? AND r2.latitude >= ? AND r2.longitude >= ? AND r2.longitude <= ?) AS res LIMIT ?,?",
    )
    .bind(user_id)
    .bind(latitude_north)
    .bind(latitude_south)
    .bind(longitude_west)
    .bind(longitude_east)
    .bind(user_id)
    .bind(latitude_north)
    .bind(latitude_south)
    .bind(longitude_west)
    .bind(longitude_east)
    .bind(lower_count)
    .bind(higher_count)
    .fetch_all(client)
    .await?;

    let out: Vec<Review> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

/// Fetches latest reviews from a given page.
/// Accounts for the passed user_id's fiends and own reviews.
/// ## Results are paged.
pub async fn get_latest_reviews(
    client: &MySqlPool,
    user_id: &str,
    page: u32,
) -> Result<Vec<Review>, Box<dyn std::error::Error>> {
    const PAGE_SIZE: u32 = 5;
    let lower_count = page * PAGE_SIZE;
    let higher_count = lower_count + PAGE_SIZE;
    let rows = sqlx::query(
        "SELECT * FROM (SELECT r1.* FROM review as r1
        INNER JOIN friend as f1 ON f1.friend_id = r1.user_id 
        WHERE f1.user_id = ?
        UNION
        SELECT r2.* FROM review as r2
        WHERE r2.user_id = ?) as res ORDER BY res.created DESC LIMIT ? OFFSET ?
        ",
    )
    .bind(user_id)
    .bind(user_id)
    .bind(lower_count)
    .bind(higher_count)
    .fetch_all(client)
    .await?;

    let out: Vec<Review> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

/// Gets all likes for a given review.
/// ## Does not validate the review is able to be viewed by calling user.
pub async fn get_all_likes(client: &MySqlPool, review_id: &str) -> Result<Vec<Like>, Error> {
    let rows = sqlx::query("SELECT * FROM likes WHERE review_id = ?")
        .bind(review_id)
        .fetch_all(client)
        .await?;

    let out: Vec<Like> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

/// Gets whether a specific review is already liked by a user.
pub async fn is_already_liked(
    client: &MySqlPool,
    user_id: &str,
    review_id: &str,
) -> Result<bool, Error> {
    let rows = sqlx::query("SELECT * FROM likes WHERE review_id = ? and user_id = ?")
        .bind(review_id)
        .bind(user_id)
        .fetch_all(client)
        .await?;

    return Ok(rows.len() > 0);
}

/// Gets all the replies for a given review.
/// ## Does not validate the review is able to be viewed by calling user.
pub async fn get_all_replies(client: &MySqlPool, review_id: &str) -> Result<Vec<Reply>, Error> {
    let rows = sqlx::query("SELECT * FROM reply WHERE review_id = ?")
        .bind(review_id)
        .fetch_all(client)
        .await?;

    let out: Vec<Reply> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}
