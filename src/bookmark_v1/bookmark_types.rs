use crate::db::Bookmark;
use serde::Serialize;

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
pub struct BookmarkPub {
    pub id: String,
    pub user_id: String,
    pub created: i64,
    pub category: String,
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl From<Bookmark> for BookmarkPub {
    fn from(bookmark: Bookmark) -> BookmarkPub {
        BookmarkPub {
            id: bookmark.id,
            user_id: bookmark.user_id,
            created: bookmark.created.timestamp_millis(),
            category: bookmark.category,
            location_name: bookmark.location_name,
            latitude: bookmark.latitude,
            longitude: bookmark.longitude,
        }
    }
}
