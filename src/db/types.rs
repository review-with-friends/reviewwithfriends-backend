use sqlx::{mysql::MySqlRow, types::chrono::NaiveDateTime, Row};

/// A unique user in the application.
pub struct User {
    /// Guid unique identifier.
    pub id: String,

    /// Unique name for a user. Used for lookup.
    pub name: String,

    /// Display name to use for the ui.
    pub display_name: String,

    /// Phone number used for authentication for this user.
    pub phone: String,

    /// Datetime the user was created.
    pub created: NaiveDateTime,

    /// Id of the profile image pic.
    /// Used to find avatar image.
    pub pic_id: String,

    pub device_token: Option<String>,

    /// Optional recovery email to be set by the user.
    pub email: Option<String>,

    /// 0 Is enabled, 1 is disabled.
    pub disabled: i8,
}

impl From<&MySqlRow> for User {
    fn from(row: &MySqlRow) -> User {
        User {
            id: row.get("id"),
            name: row.get("name"),
            display_name: row.get("display_name"),
            phone: row.get("phone"),
            created: row.get("created"),
            pic_id: row.get("pic_id"),
            device_token: row.get("device_token"),
            email: row.get("email"),
            disabled: row.get("disabled"),
        }
    }
}

/// Represents a specific phone authentication attempt.
pub struct PhoneAuth {
    /// Guid unique identifier.
    pub id: String,

    /// Phone Number auth request was sent to.
    pub phone: String,

    /// Datetime in UTC the PhoneAuth attempt was started.
    pub created: NaiveDateTime,

    /// IP address the request was sent from.
    pub ip: String,

    /// The 9 digit code used to compare against for authentication.
    pub code: String,

    /// Whether the PhoneAuth was used for login.
    pub used: bool,
}

impl From<&MySqlRow> for PhoneAuth {
    fn from(row: &MySqlRow) -> PhoneAuth {
        PhoneAuth {
            id: row.get("id"),
            phone: row.get("phone"),
            created: row.get("created"),
            ip: row.get("ip"),
            code: row.get("code"),
            used: row.get("used"),
        }
    }
}

/// Represents an attempt to use a code provided by phone auth.
pub struct AuthAttempt {
    /// Guid unique identifier.
    pub id: String,

    /// Phone number used to auth against.
    pub phone: String,

    /// Datetime the request was made.
    pub created: NaiveDateTime,
}

impl From<&MySqlRow> for AuthAttempt {
    fn from(row: &MySqlRow) -> AuthAttempt {
        AuthAttempt {
            id: row.get("id"),
            phone: row.get("phone"),
            created: row.get("created"),
        }
    }
}

/// Represents one direction of a friend relationship.
/// In a logical friendship, two friend records exist
/// with user_id and friend_id flipped.
pub struct Friend {
    /// Guid unique identifier.
    pub id: String,

    /// Datetime the request was made.
    pub created: NaiveDateTime,

    /// Primary user for the friendship.
    pub user_id: String,

    /// Secondary user for the friendship.
    pub friend_id: String,
}

impl From<&MySqlRow> for Friend {
    fn from(row: &MySqlRow) -> Friend {
        Friend {
            id: row.get("id"),
            created: row.get("created"),
            user_id: row.get("user_id"),
            friend_id: row.get("friend_id"),
        }
    }
}

/// Represents a sent friend request.
/// user_id is the sender
pub struct FriendRequest {
    /// Guid unique identifier.
    pub id: String,

    /// Datetime the request was made.
    pub created: NaiveDateTime,

    /// The sender of the friendship reqest.
    pub user_id: String,

    /// The receiver of the friendship request.
    pub friend_id: String,

    /// friend_id is able to mark this request as ignored.
    pub ignored: bool,
}

impl From<&MySqlRow> for FriendRequest {
    fn from(row: &MySqlRow) -> FriendRequest {
        FriendRequest {
            id: row.get("id"),
            created: row.get("created"),
            user_id: row.get("user_id"),
            friend_id: row.get("friend_id"),
            ignored: row.get("ignored"),
        }
    }
}

/// Represents an uploaded image.
pub struct Pic {
    /// Guid unique identifier.
    pub id: String,

    /// To support multiple reviews pics, pics can relate to multiple images.
    pub review_id: Option<String>,

    /// Datetime the request was made.
    pub created: NaiveDateTime,

    /// The handler used to fetch/create the image.
    /// The handler must use the id to find the image.
    pub pic_handler: u8,

    /// Width of pic in pixels
    pub width: u16,

    /// Height of pic in pixels
    pub height: u16,
}

impl From<&MySqlRow> for Pic {
    fn from(row: &MySqlRow) -> Pic {
        Pic {
            id: row.get("id"),
            review_id: row.get("review_id"),
            created: row.get("created"),
            pic_handler: row.get("pic_handler"),
            width: row.get("width"),
            height: row.get("height"),
        }
    }
}

/// Represents a review for a location.
pub struct Review {
    /// Guid unique identifier.
    pub id: String,

    // user who made the post
    pub user_id: String,

    /// Datetime the request was made.
    pub created: NaiveDateTime,

    // category of the review, i.e. restaurant or cafe
    pub category: String,

    // Actual review text.
    pub text: String,

    // 0 - 5 stars for review.
    pub stars: u8,

    // name of the location as per apple maps
    pub location_name: String,

    // Latitude of the reviewed location
    pub latitude: f64,

    // Longitude of the reviewed location
    pub longitude: f64,

    // custom locations
    pub is_custom: bool,
}

impl From<&MySqlRow> for Review {
    fn from(row: &MySqlRow) -> Review {
        Review {
            id: row.get("id"),
            user_id: row.get("user_id"),
            created: row.get("created"),
            category: row.get("category"),
            text: row.get("text"),
            stars: row.get("stars"),
            location_name: row.get("location_name"),
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            is_custom: row.get("is_custom"),
        }
    }
}

/// Represents a review for a location.
pub struct ReviewAnnotation {
    /// Guid unique identifier.
    pub id: String,

    // user who made the post
    pub user_id: String,

    /// Datetime the request was made.
    pub created: NaiveDateTime,

    /// Id of the associated pic record.
    pub pic_id: String,

    /// Where to find the pic.
    pub pic_handler: u8,

    // category of the review, i.e. restaurant or cafe
    pub category: String,

    // name of the location as per apple maps
    pub location_name: String,

    // Latitude of the reviewed location
    pub latitude: f64,

    // Longitude of the reviewed location
    pub longitude: f64,

    // custom locations
    pub is_custom: bool,
}

impl From<&MySqlRow> for ReviewAnnotation {
    fn from(row: &MySqlRow) -> ReviewAnnotation {
        ReviewAnnotation {
            id: row.get("id"),
            user_id: row.get("user_id"),
            created: row.get("created"),
            pic_id: row.get("pic_id"),
            pic_handler: row.get("pic_handler"),
            category: row.get("category"),
            location_name: row.get("location_name"),
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            is_custom: row.get("is_custom"),
        }
    }
}

/// Represents a like for a post.
pub struct Like {
    /// Guid unique identifier.
    pub id: String,

    /// Datetime the like was made.
    pub created: NaiveDateTime,

    /// The user who liked the review.
    pub user_id: String,

    /// Id for the review that was liked.
    pub review_id: String,
}

/// Represents a reply to a post.
pub struct Reply {
    /// Guid unique identifier.
    pub id: String,

    /// Datetime the like was made.
    pub created: NaiveDateTime,

    /// The user who liked the review.
    pub user_id: String,

    /// Id of the review the reply is for.
    pub review_id: String,

    /// Text for the reply.
    pub text: String,

    /// Id of the reply this reply is replying to.
    pub reply_to_id: Option<String>,
}

impl From<&MySqlRow> for Reply {
    fn from(row: &MySqlRow) -> Reply {
        Reply {
            id: row.get("id"),
            created: row.get("created"),
            user_id: row.get("user_id"),
            review_id: row.get("review_id"),
            text: row.get("text"),
            reply_to_id: row.get("reply_to_id"),
        }
    }
}

/// Represents a notification to a post.
pub struct Notification {
    /// Guid unique identifier.
    pub id: String,

    /// Datetime the notification was made.
    pub created: NaiveDateTime,

    /// The user who is responsible for receiving the notification.
    pub review_user_id: String,

    // Id of the review the notification is for.
    pub review_id: String,

    /// The user who is responsible for the notification.
    pub user_id: String,

    /// The type of action which created the notification.
    pub action_type: u8,
}

impl From<&MySqlRow> for Notification {
    fn from(row: &MySqlRow) -> Notification {
        Notification {
            id: row.get("id"),
            created: row.get("created"),
            review_user_id: row.get("review_user_id"),
            user_id: row.get("user_id"),
            review_id: row.get("review_id"),
            action_type: row.get("action_type"),
        }
    }
}

/// Represents a notification to a post.
pub struct ExpandedNotification {
    /// Guid unique identifier.
    pub id: String,

    /// Datetime the notification was made.
    pub created: NaiveDateTime,

    /// The user who is responsible for receiving the notification.
    pub review_user_id: String,

    // Id of the review the notification is for.
    pub review_id: String,

    /// The user who is responsible for the notification.
    pub user_id: String,

    /// The type of action which created the notification.
    pub action_type: u8,

    pub review_location: String,
}

impl From<&MySqlRow> for ExpandedNotification {
    fn from(row: &MySqlRow) -> ExpandedNotification {
        ExpandedNotification {
            id: row.get("id"),
            created: row.get("created"),
            review_user_id: row.get("review_user_id"),
            user_id: row.get("user_id"),
            review_id: row.get("review_id"),
            action_type: row.get("action_type"),
            review_location: row.get("review_location"),
        }
    }
}

/// Represents a report against a user.
pub struct Report {
    /// Guid unique identifier.
    pub id: String,

    /// Datetime the report was made.
    pub created: NaiveDateTime,

    /// The user who got reported
    pub user_id: String,

    // Id of the user who reported
    pub reporter_id: String,

    /// The type of report.
    pub report_type: u8,
}
