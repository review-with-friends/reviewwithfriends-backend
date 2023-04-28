use crate::{db::User, pic_v1::get_digital_ocean_url};
use serde::Serialize;

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
pub struct UserPub {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub created: i64,
    pub pic_id: String,
    pub recovery: bool,
    pub pic_url: String,
}

impl From<User> for UserPub {
    fn from(user: User) -> UserPub {
        UserPub {
            id: user.id,
            name: user.name,
            display_name: user.display_name,
            created: user.created.timestamp_millis(),
            pic_id: user.pic_id.clone(),
            recovery: user.email.is_some(),
            pic_url: get_digital_ocean_url(&user.pic_id),
        }
    }
}
