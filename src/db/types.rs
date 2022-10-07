use chrono::{DateTime, Utc};

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
    pub created: DateTime<Utc>,
}

/// Represents a specific phone authentication attempt.
pub struct PhoneAuth {
    /// Guid unique identifier.
    pub id: String,

    /// Phone Number auth request was sent to.
    pub phone: String,

    /// Datetime in UTC the PhoneAuth attempt was started.
    pub created: DateTime<Utc>,

    /// IP address the request was sent from.
    pub ip: String,

    /// The 9 digit code used to compare against for authentication.
    pub code: String,

    /// Whether the PhoneAuth was used for login.
    pub used: bool,
}

/// Represents an attempt to use a code provided by phone auth.
pub struct AuthAttempt {
    /// Guid unique identifier.
    pub id: String,

    /// Phone number used to auth against.
    pub phone: String,

    /// Datetime the request was made.
    pub created: DateTime<Utc>,
}
