use chrono::NaiveDateTime;
use serde::Serialize;

use crate::db::User;

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
pub struct UserPub {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub created: NaiveDateTime,
    pub pic_id: String,
}

impl From<User> for UserPub {
    fn from(user: User) -> UserPub {
        UserPub {
            id: user.id,
            name: user.name,
            display_name: user.display_name,
            created: user.created,
            pic_id: user.pic_id,
        }
    }
}
