use rocket_db_pools::sqlx::{mysql::MySqlRow, Row};
use sqlx::types::chrono::NaiveDateTime;

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
}

impl From<&MySqlRow> for User {
    fn from(row: &MySqlRow) -> User {
        User {
            id: row.get("id"),
            name: row.get("name"),
            display_name: row.get("display_name"),
            phone: row.get("phone"),
            created: row.get("created"),
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
