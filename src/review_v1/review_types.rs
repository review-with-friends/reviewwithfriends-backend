use chrono::NaiveDateTime;
use serde::Serialize;

use crate::db::Review;

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
pub struct ReviewPub {
    pub id: String,
    pub user_id: String,
    pub created: NaiveDateTime,
    pub pic_id: Option<String>,
    pub category: String,
    pub text: String,
    pub stars: u8,
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_custom: bool,
}

impl From<Review> for ReviewPub {
    fn from(review: Review) -> ReviewPub {
        ReviewPub {
            id: review.id,
            user_id: review.user_id,
            created: review.created,
            pic_id: review.pic_id,
            category: review.category,
            text: review.text,
            stars: review.stars,
            location_name: review.location_name,
            latitude: review.latitude,
            longitude: review.longitude,
            is_custom: review.is_custom,
        }
    }
}
