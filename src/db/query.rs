use chrono::Duration;
use sqlx::{types::chrono::Utc, Error, MySqlPool};

use super::{
    AuthAttempt, Bookmark, ExpandedNotification, Friend, FriendRequest, Like, PhoneAuth, Pic,
    Reply, Review, ReviewAnnotation, User,
};

/// All query text constants defined in this file should be formatted with the following tool:
/// https://www.dpriver.com/pp/sqlformat.htm

/// Simply gets a ping record.
pub async fn get_ping(client: &MySqlPool, id: &str) -> Result<String, Error> {
    struct Ping {
        id: String,
    }
    let ping = sqlx::query_as!(
        Ping,
        "SELECT *
        FROM   ping
        WHERE  id = ? ",
        id
    )
    .fetch_one(client)
    .await?;

    return Ok(ping.id);
}

/// Tries to get a user by `user.id`, and will return `None` if not found.
pub async fn get_user(client: &MySqlPool, id: &str) -> Result<Option<User>, Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT *
        FROM   user
        WHERE  id = ? ",
        id
    )
    .fetch_optional(client)
    .await?;

    return Ok(user);
}

/// Tries to get a user by exact `user.name`, and will return `None` if not found.
pub async fn get_user_from_name(client: &MySqlPool, name: &str) -> Result<Option<User>, Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT *
        FROM   user
        WHERE  name = ? ",
        name
    )
    .fetch_optional(client)
    .await?;

    return Ok(user);
}

/// Gets a list of users from the given `user.name`.
/// This search is a trailing wildcard and limits to top 50 results.
pub async fn search_user_from_name(client: &MySqlPool, name: &str) -> Result<Vec<User>, Error> {
    let search_term = format!("{}%", name.replace("%", ""));
    let users = sqlx::query_as!(
        User,
        "SELECT *
        FROM   user
        WHERE  name LIKE ?
        LIMIT  50 ",
        search_term,
    )
    .fetch_all(client)
    .await?;

    return Ok(users);
}

/// Gets the existence of a user by `user.id`. True if exists, false if not.
pub async fn does_user_exist(client: &MySqlPool, id: &str) -> Result<bool, Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT *
        FROM   user
        WHERE  id = ? ",
        id
    )
    .fetch_all(client)
    .await?;

    return Ok(users.len() == 1);
}

/// Gets the existence of a user by `user.name`. True if exists, false if not.
pub async fn does_user_exist_by_name(client: &MySqlPool, name: &str) -> Result<bool, Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT *
        FROM   user
        WHERE  name = ? ",
        name
    )
    .fetch_all(client)
    .await?;

    return Ok(users.len() == 1);
}

/// Gets a given user by `user.phone`, and will return `None` if not found.
pub async fn get_user_by_phone(client: &MySqlPool, phone: &str) -> Result<Option<User>, Error> {
    let mut rows = sqlx::query_as!(
        User,
        "SELECT *
        FROM user
        WHERE phone = ? ",
        phone
    )
    .fetch_all(client)
    .await?;

    return Ok(rows.pop());
}

/// Gets the current phoneauths.
/// Results are within the last 1 hour of `phoneauth.created`, and `phoneauth.used` is `false`
pub async fn get_current_phoneauths(
    client: &MySqlPool,
    phone: &str,
) -> Result<Vec<PhoneAuth>, Error> {
    let phone_auths = sqlx::query_as!(
        PhoneAuth,
        "SELECT *
        FROM   phoneauth
        WHERE  created > ?
               AND phone = ?
               AND used = false",
        Utc::now().naive_utc() - Duration::hours(1),
        phone
    )
    .fetch_all(client)
    .await?;

    return Ok(phone_auths);
}

/// Gets the current authattempts for a given `authattempt.phone` in the last 1 hour of `authattempt.created`.
pub async fn get_phoneauth_attempts(
    client: &MySqlPool,
    phone: &str,
) -> Result<Vec<AuthAttempt>, Error> {
    let auth_attempts = sqlx::query_as!(
        AuthAttempt,
        "SELECT *
        FROM   authattempt
        WHERE  created > ?
            AND phone = ? ",
        Utc::now().naive_utc() - Duration::hours(1),
        phone
    )
    .fetch_all(client)
    .await?;

    return Ok(auth_attempts);
}

/// Gets the users current incoming friend requests that `friendrequest.ignored` is false.
pub async fn get_incoming_friend_requests(
    client: &MySqlPool,
    user_id: &str,
) -> Result<Vec<FriendRequest>, Error> {
    let friend_requests = sqlx::query_as!(
        FriendRequest,
        "SELECT *
        FROM   friendrequest
        WHERE  friend_id = ?
            AND ignored = false ",
        user_id
    )
    .fetch_all(client)
    .await?;

    return Ok(friend_requests);
}

/// Gets the users current incoming friend requests that can be accepted. This included requests that have been ignored.
pub async fn get_acceptable_friend_requests(
    client: &MySqlPool,
    user_id: &str,
) -> Result<Vec<FriendRequest>, Error> {
    let friend_requests = sqlx::query_as!(
        FriendRequest,
        "SELECT *
        FROM   friendrequest
        WHERE  friend_id = ? ",
        user_id
    )
    .fetch_all(client)
    .await?;

    return Ok(friend_requests);
}

/// Gets the users current incoming friend requests that `friendrequest.ignored` is true.
pub async fn get_incoming_ignored_friend_requests(
    client: &MySqlPool,
    user_id: &str,
) -> Result<Vec<FriendRequest>, Error> {
    let friend_requests = sqlx::query_as!(
        FriendRequest,
        "SELECT *
        FROM   friendrequest
        WHERE  friend_id = ?
            AND ignored = true ",
        user_id
    )
    .fetch_all(client)
    .await?;

    return Ok(friend_requests);
}

/// Gets the users outgoing friend requests.
pub async fn get_outgoing_friend_requests(
    client: &MySqlPool,
    user_id: &str,
) -> Result<Vec<FriendRequest>, Error> {
    let friend_requests = sqlx::query_as!(
        FriendRequest,
        "SELECT *
        FROM   friendrequest
        WHERE  user_id = ? ",
        user_id
    )
    .fetch_all(client)
    .await?;

    return Ok(friend_requests);
}

/// Gets the users current friends list.
/// Not paged.
pub async fn get_current_friends(client: &MySqlPool, user_id: &str) -> Result<Vec<Friend>, Error> {
    let friends = sqlx::query_as!(
        Friend,
        "SELECT *
        FROM   friend
        WHERE  user_id = ? ",
        user_id
    )
    .fetch_all(client)
    .await?;

    return Ok(friends);
}

/// Gets a pic record for a specific id, will return `None` if it doesn't exists.
pub async fn get_pic(client: &MySqlPool, id: &str) -> Result<Option<Pic>, Error> {
    let pic_opt = sqlx::query_as!(
        Pic,
        "SELECT *
        FROM   pic
        WHERE  id = ? ",
        id
    )
    .fetch_optional(client)
    .await?;

    return Ok(pic_opt);
}

/// Gets a single review for the given review_id.
/// Accounts for the passed user_id's fiends and own reviews.
pub async fn get_review(
    client: &MySqlPool,
    user_id: &str,
    review_id: &str,
) -> Result<Option<Review>, Error> {
    let review_opt = sqlx::query_as!(
        Review,
        "SELECT r1.id,
        r1.user_id,
        r1.created,
        r1.category,
        r1.text,
        r1.stars,
        r1.location_name,
        ST_X(r1.location) as longitude,
        ST_Y(r1.location) as latitude,
        r1.is_custom,
        r1.delivered,
        r1.recommended
        FROM   review AS r1
               INNER JOIN friend AS f1
                       ON f1.friend_id = r1.user_id
        WHERE  f1.user_id = ?
               AND r1.id = ?
        UNION
        SELECT r2.id,
        r2.user_id,
        r2.created,
        r2.category,
        r2.text,
        r2.stars,
        r2.location_name,
        ST_X(r2.location) as longitude,
        ST_Y(r2.location) as latitude,
        r2.is_custom,
        r2.delivered,
        r2.recommended
        FROM   review AS r2
        WHERE  r2.user_id = ?
               AND r2.id = ? ",
        user_id,
        review_id,
        user_id,
        review_id
    )
    .fetch_optional(client)
    .await?;

    return Ok(review_opt);
}

/// Gets all reviews from a given name, latitude, and longitude combination.
/// Accounts for the passed user_id's fiends and own reviews.
/// ## Results are NOT paged.
pub async fn get_reviews_from_location(
    client: &MySqlPool,
    user_id: &str,
    name: &str,
    latitude: f64,
    longitude: f64,
) -> Result<Vec<Review>, Error> {
    const ACCURACY_SIZE: f64 = 0.001;
    let reviews = sqlx::query_as!(
        Review,
        "SELECT r.id,
        r.user_id,
        r.created,
        r.category,
        r.text,
        r.stars,
        r.location_name,
        ST_X(r.location) as longitude,
        ST_Y(r.location) as latitude,
        r.is_custom,
        r.delivered,
        r.recommended
        FROM   review AS r
               INNER JOIN friend AS f
                       ON r.user_id = f.friend_id
        WHERE  f.user_id = ?
               AND r.location_name = ?
               AND ST_Contains(ST_Buffer(POINT(?, ?), ?), r.location) = 1",
        user_id,
        name,
        longitude,
        latitude,
        ACCURACY_SIZE
    )
    .fetch_all(client)
    .await?;

    return Ok(reviews);
}

/// Gets all reviews from a user.
/// Accounts for the passed user_id's fiends and own reviews.
pub async fn get_reviews_from_user(
    client: &MySqlPool,
    user_id: &str,
    target_user_id: &str,
    page: u32,
) -> Result<Vec<Review>, Error> {
    const PAGE_SIZE: u32 = 5;

    let lower_count = page * PAGE_SIZE;

    let reviews = sqlx::query_as!(
        Review,
        "SELECT r.id,
        r.user_id,
        r.created,
        r.category,
        r.text,
        r.stars,
        r.location_name,
        ST_X(r.location) as longitude,
        ST_Y(r.location) as latitude,
        r.is_custom,
        r.delivered,
        r.recommended
        FROM   review AS r
               INNER JOIN friend AS f
                       ON r.user_id = f.friend_id
        WHERE  f.user_id = ?
            AND r.user_id = ?
        ORDER BY r.created DESC
        LIMIT  ? offset ? ",
        user_id,
        target_user_id,
        PAGE_SIZE,
        lower_count
    )
    .fetch_all(client)
    .await?;

    return Ok(reviews);
}

/// Gets all recommended reviews from a user.
/// Accounts for the passed user_id's fiends and own reviews.
pub async fn get_recommended_reviews_from_user(
    client: &MySqlPool,
    user_id: &str,
    target_user_id: &str,
    page: u32,
) -> Result<Vec<Review>, Error> {
    const PAGE_SIZE: u32 = 5;

    let lower_count = page * PAGE_SIZE;

    let reviews = sqlx::query_as!(
        Review,
        "SELECT r.id,
        r.user_id,
        r.created,
        r.category,
        r.text,
        r.stars,
        r.location_name,
        ST_X(r.location) as longitude,
        ST_Y(r.location) as latitude,
        r.is_custom,
        r.delivered,
        r.recommended
        FROM   review AS r
               INNER JOIN friend AS f
                       ON r.user_id = f.friend_id
        WHERE  f.user_id = ?
            AND r.user_id = ?
            AND r.recommended = true
        ORDER BY r.created DESC
        LIMIT  ? offset ? ",
        user_id,
        target_user_id,
        PAGE_SIZE,
        lower_count
    )
    .fetch_all(client)
    .await?;

    return Ok(reviews);
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
) -> Result<Vec<ReviewAnnotation>, Error> {
    const PAGE_SIZE: u32 = 100;
    let lower_count = page * PAGE_SIZE;

    let annotations = sqlx::query_as!(
        ReviewAnnotation,
        "SELECT r.id,
        r.user_id,
        r.created,
        p.id             AS pic_id,
        p.pic_handler   AS pic_handler,
        r.category,
        r.location_name,
        St_x(r.location) AS longitude,
        St_y(r.location) AS latitude,
        r.is_custom,
        r.delivered,
        r.recommended
 FROM   review AS r
        INNER JOIN friend AS f
                ON r.user_id = f.friend_id
        LEFT JOIN pic AS p
               ON p.id = (SELECT id
                          FROM   pic pp
                          WHERE  pp.review_id = r.id
                          LIMIT  1)
 WHERE  f.user_id = ?
        AND St_contains(St_makeenvelope(Point(?, ?), Point(?, ?)), r.location)
 LIMIT  ? offset ? ",
        user_id,
        longitude_west,
        latitude_north,
        longitude_east,
        latitude_south,
        PAGE_SIZE,
        lower_count
    )
    .fetch_all(client)
    .await?;

    return Ok(annotations);
}

/// Gets reviews from a given bounding box.
/// Accounts for the passed user_id's friends and own reviews.
/// ## Results are paged.
pub async fn get_reviews_from_bounds_with_exclusions(
    client: &MySqlPool,
    user_id: &str,
    latitude_north: f64,
    latitude_south: f64,
    longitude_west: f64,
    longitude_east: f64,
    latitude_north_e: f64,
    latitude_south_e: f64,
    longitude_west_e: f64,
    longitude_east_e: f64,
    page: u32,
) -> Result<Vec<ReviewAnnotation>, Error> {
    const PAGE_SIZE: u32 = 100;
    let lower_count = page * PAGE_SIZE;

    let annotations = sqlx::query_as!(
        ReviewAnnotation,
        "SELECT r.id,
        r.user_id,
        r.created,
        p.id             AS pic_id,
        p.pic_handler   AS pic_handler,
        r.category,
        r.location_name,
        St_x(r.location) AS longitude,
        St_y(r.location) AS latitude,
        r.is_custom,
        r.delivered,
        r.recommended
 FROM   review AS r
        INNER JOIN friend AS f
                ON r.user_id = f.friend_id
        LEFT JOIN pic AS p
               ON p.id = (SELECT id
                          FROM   pic pp
                          WHERE  pp.review_id = r.id
                          LIMIT  1)
 WHERE  f.user_id = ?
        AND St_contains(St_makeenvelope(Point(?, ?), Point(?, ?)), r.location)
        AND NOT St_contains(St_makeenvelope(Point(?, ?), Point(?, ?)),
                r.location)
 LIMIT  ? offset ? ",
        user_id,
        longitude_west,
        latitude_north,
        longitude_east,
        latitude_south,
        longitude_west_e,
        latitude_north_e,
        longitude_east_e,
        latitude_south_e,
        PAGE_SIZE,
        lower_count
    )
    .fetch_all(client)
    .await?;

    return Ok(annotations);
}

/// Fetches latest reviews from a given page.
/// Accounts for the passed user_id's fiends and own reviews.
/// ## Results are paged.
pub async fn get_latest_reviews(
    client: &MySqlPool,
    user_id: &str,
    page: u32,
) -> Result<Vec<Review>, Error> {
    const PAGE_SIZE: u32 = 5;

    let lower_count = page * PAGE_SIZE;

    let reviews = sqlx::query_as!(
        Review,
        "SELECT r.id,
        r.user_id,
        r.created,
        r.category,
        r.text,
        r.stars,
        r.location_name,
        St_x(r.location) AS longitude,
        St_y(r.location) AS latitude,
        r.is_custom,
        r.delivered,
        r.recommended
 FROM   review AS r
        INNER JOIN friend AS f
                ON r.user_id = f.friend_id
 WHERE  f.user_id = ?
 ORDER  BY r.created DESC
 LIMIT  ? offset ? ",
        user_id,
        PAGE_SIZE,
        lower_count
    )
    .fetch_all(client)
    .await?;

    return Ok(reviews);
}

/// Fetches latest reviews from a given page.
/// Accounts for the passed user_id's fiends and own reviews.
/// ## Results are paged.
pub async fn search_latest_reviews(
    client: &MySqlPool,
    user_id: &str,
    search_prefix: &str,
    page: u32,
) -> Result<Vec<Review>, Error> {
    const PAGE_SIZE: u32 = 5;
    let lower_count = page * PAGE_SIZE;
    let search_term = format!("{}%", search_prefix.replace("%", ""));

    let reviews = sqlx::query_as!(
        Review,
        "SELECT r.id,
        r.user_id,
        r.created,
        r.category,
        r.text,
        r.stars,
        r.location_name,
        St_x(r.location) AS longitude,
        St_y(r.location) AS latitude,
        r.is_custom,
        r.delivered,
        r.recommended
 FROM   review AS r
        INNER JOIN friend AS f
                ON r.user_id = f.friend_id
 WHERE  f.user_id = ?
 AND r.location_name LIKE ?
 ORDER  BY r.created DESC
 LIMIT  ? offset ? ",
        user_id,
        search_term,
        PAGE_SIZE,
        lower_count
    )
    .fetch_all(client)
    .await?;

    return Ok(reviews);
}

/// Gets all likes for a given review.
/// ## Does not validate the review is able to be viewed by calling user.
pub async fn get_all_likes(client: &MySqlPool, review_id: &str) -> Result<Vec<Like>, Error> {
    let rows = sqlx::query_as!(Like, "SELECT * FROM likes WHERE  review_id = ? ", review_id)
        .fetch_all(client)
        .await?;

    return Ok(rows);
}

/// Gets all likes that a user has made
pub async fn get_liked_reviews(
    client: &MySqlPool,
    user_id: &str,
    page: u32,
) -> Result<Vec<Review>, Error> {
    const PAGE_SIZE: u32 = 5;
    let lower_count = page * PAGE_SIZE;

    let reviews = sqlx::query_as!(
        Review,
        "SELECT r.id,
            r.user_id,
            r.created,
            r.category,
            r.text,
            r.stars,
            r.location_name,
            St_x(r.location) AS longitude,
            St_y(r.location) AS latitude,
            r.is_custom,
            r.delivered,
            r.recommended
            FROM   review as r
        INNER JOIN likes as l on r.id = l.review_id
            WHERE  l.user_id = ?
        ORDER  BY l.created DESC
        LIMIT  ? offset ? ",
        user_id,
        PAGE_SIZE,
        lower_count
    )
    .fetch_all(client)
    .await?;

    return Ok(reviews);
}

/// Gets whether a specific review is already liked by a user.
pub async fn is_already_liked(
    client: &MySqlPool,
    user_id: &str,
    review_id: &str,
) -> Result<bool, Error> {
    let likes = sqlx::query_as!(
        Like,
        "SELECT *
        FROM   likes
        WHERE  review_id = ?
            AND user_id = ? ",
        review_id,
        user_id
    )
    .fetch_all(client)
    .await?;

    return Ok(likes.len() > 0);
}

/// Gets all the replies for a given review.
/// ## Does not validate the review is able to be viewed by calling user.
pub async fn get_all_replies(client: &MySqlPool, review_id: &str) -> Result<Vec<Reply>, Error> {
    let replies = sqlx::query_as!(
        Reply,
        "SELECT *
        FROM   reply
        WHERE  review_id = ? ",
        review_id
    )
    .fetch_all(client)
    .await?;

    return Ok(replies);
}

/// Gets all the pics for a given review.
/// ## Does not validate the review is able to be viewed by calling user.
pub async fn get_all_pics(client: &MySqlPool, review_id: &str) -> Result<Vec<Pic>, Error> {
    let pics = sqlx::query_as!(
        Pic,
        "SELECT *
        FROM   pic
        WHERE  review_id = ? ",
        review_id
    )
    .fetch_all(client)
    .await?;

    return Ok(pics);
}

/// Gets the top 50 latest notifications for the user.
pub async fn get_notifications(
    client: &MySqlPool,
    user_id: &str,
) -> Result<Vec<ExpandedNotification>, Error> {
    let notifications = sqlx::query_as!(
        ExpandedNotification,
        "SELECT n.id, n.created, n.review_user_id, n.user_id, n.review_id, n.action_type, r.location_name AS review_location
        FROM   notification AS n
        INNER JOIN review as r on r.id = n.review_id
    WHERE  n.review_user_id = ?
        ORDER BY n.created DESC
    LIMIT 50",
    user_id
    )
    .fetch_all(client)
    .await?;

    return Ok(notifications);
}

/// Gets the count of notifications the user has pending.
pub async fn get_notification_count(client: &MySqlPool, user_id: &str) -> i64 {
    struct Count {
        count: i64,
    }

    let count_res = sqlx::query_as!(
        Count,
        "SELECT count(*) as count FROM notification WHERE review_user_id = ?",
        user_id
    )
    .fetch_one(client)
    .await;

    match count_res {
        Ok(count) => return count.count,
        Err(_) => return 0,
    }
}

/// Searches for users that match any of the given phone numbers.
/// It's expected the caller randomize the ordering to ensure
/// reverse lookups are not as efficient.
pub async fn phone_number_discovery(
    client: &MySqlPool,
    numbers: &Vec<&str>,
) -> Result<Vec<User>, Error> {
    let params = format!("?{}", ", ?".repeat(numbers.len() - 1));
    let query_str = format!("SELECT * FROM user WHERE phone IN ( {} )", params);

    let mut query = sqlx::query(&query_str);
    for i in numbers {
        query = query.bind(i);
    }
    let rows = query.fetch_all(client).await?;

    let out: Vec<User> = rows.iter().map(|row| row.into()).collect();

    return Ok(out);
}

/// Gets whether users are friends with eachother.
/// Order of user_id params doesn't really matter.
pub async fn are_users_friends(
    client: &MySqlPool,
    user_id: &str,
    other_user_id: &str,
) -> Result<bool, Error> {
    let friends = sqlx::query_as!(
        Friend,
        "SELECT *
        FROM   friend
        WHERE  user_id = ?
            AND friend_id = ? ",
        user_id,
        other_user_id
    )
    .fetch_all(client)
    .await?;

    return Ok(friends.len() > 0);
}

/// Gets a reply with a specific Id from a specific review.
pub async fn get_reply(
    client: &MySqlPool,
    review_id: &str,
    reply_id: &str,
) -> Result<Option<Reply>, Error> {
    let reply = sqlx::query_as!(
        Reply,
        "SELECT *
        FROM   reply
        WHERE  id = ?
            AND review_id = ?",
        reply_id,
        review_id
    )
    .fetch_optional(client)
    .await?;

    return Ok(reply);
}

/// Gets all bookmarks fromm a friend of this user.
/// Accounts for the passed user_id's friends and own bookmarks.
pub async fn get_all_bookmarks(
    client: &MySqlPool,
    user_id: &str,
    target_user_id: &str,
) -> Result<Vec<Bookmark>, Error> {
    let bookmarks = sqlx::query_as!(
        Bookmark,
        "SELECT bm.id,
        bm.user_id,
        bm.created,
        bm.category,
        bm.location_name,
        ST_X(bm.location) as longitude,
        ST_Y(bm.location) as latitude
        FROM   bookmark AS bm
               INNER JOIN friend AS f
                       ON bm.user_id = f.friend_id
        WHERE  f.user_id = ?
            AND bm.user_id = ?
        ORDER BY bm.created DESC",
        user_id,
        target_user_id
    )
    .fetch_all(client)
    .await?;

    return Ok(bookmarks);
}

/// Gets the existence of a bookmark by `bookmark.user_id` and `bookmark.id`. True if exists, false if not.
pub async fn does_bookmark_exist(
    client: &MySqlPool,
    user_id: &str,
    location_name: &str,
    latitude: f64,
    longitude: f64,
) -> Result<bool, Error> {
    const ACCURACY_SIZE: f64 = 0.001;
    let bookmarks = sqlx::query_as!(
        Bookmark,
        "SELECT r.id,
        r.user_id,
        r.created,
        r.category,
        r.location_name,
        ST_X(r.location) as longitude,
        ST_Y(r.location) as latitude
        FROM   bookmark AS r
               INNER JOIN friend AS f
                       ON r.user_id = f.friend_id
        WHERE  f.user_id = ?
               AND r.location_name = ?
               AND ST_Contains(ST_Buffer(POINT(?, ?), ?), r.location) = 1",
        user_id,
        location_name,
        longitude,
        latitude,
        ACCURACY_SIZE
    )
    .fetch_all(client)
    .await?;

    return Ok(bookmarks.len() >= 1);
}

/// Gets all bookmarks from a nearby radius
/// Accounts for the passed user_id's fiends and own bookmarks.
pub async fn get_nearby_bookmarks(
    client: &MySqlPool,
    calling_user_id: &str,
    user_id: &str,
    page: u32,
    latitude: f64,
    longitude: f64,
) -> Result<Vec<Bookmark>, Error> {
    const PAGE_SIZE: u32 = 5;
    let lower_count = page * PAGE_SIZE;

    const ACCURACY_SIZE: f64 = 0.025;

    let reviews = sqlx::query_as!(
        Bookmark,
        "SELECT bm.id,
        bm.user_id,
        bm.created,
        bm.category,
        bm.location_name,
        ST_X(bm.location) as longitude,
        ST_Y(bm.location) as latitude
        FROM   bookmark AS bm
               INNER JOIN friend AS f
                       ON bm.user_id = f.friend_id
        WHERE  f.user_id = ?
            AND bm.user_id = ?
               AND ST_Contains(ST_Buffer(POINT(?, ?), ?), bm.location) = 1
            ORDER BY bm.created DESC
               LIMIT  ? offset ? ",
        calling_user_id,
        user_id,
        longitude,
        latitude,
        ACCURACY_SIZE,
        PAGE_SIZE,
        lower_count
    )
    .fetch_all(client)
    .await?;

    return Ok(reviews);
}
