use crate::db::Pic;
use serde::Serialize;

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
pub struct PicPub {
    pub id: String,
    pub review_id: Option<String>,
    pub created: i64,
    pub width: u16,
    pub height: u16,
    pub url: String,
}

impl From<Pic> for PicPub {
    fn from(pic: Pic) -> PicPub {
        PicPub {
            id: pic.id.clone(),
            review_id: pic.review_id,
            created: pic.created.timestamp_millis(),
            width: pic.width,
            height: pic.height,
            url: PicPub::get_url(&pic.id, pic.pic_handler),
        }
    }
}

impl PicPub {
    pub fn get_url(pic_id: &str, pic_handler: u8) -> String {
        match pic_handler {
            1 => {
                return format!("https://bout.sfo3.cdn.digitaloceanspaces.com/{}", pic_id);
            }
            _ => {
                return format!("https://bout.sfo3.cdn.digitaloceanspaces.com/{}", pic_id);
            }
        }
    }
}
