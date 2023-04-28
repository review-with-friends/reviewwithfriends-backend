use crate::{
    db::{Review, ReviewAnnotation},
    pic_v1::PicPub,
};
use serde::Serialize;

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
pub struct ReviewPub {
    pub id: String,
    pub user_id: String,
    pub created: i64,
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
            created: review.created.timestamp_millis(),
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

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
pub struct ReviewAnnotationPub {
    pub id: String,
    pub user_id: String,
    pub created: i64,
    pub text: String,
    pub stars: u8,
    pub pic_id: String,
    pub pic_url: String,
    pub category: String,
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_custom: bool,
}

impl From<ReviewAnnotation> for ReviewAnnotationPub {
    fn from(review: ReviewAnnotation) -> ReviewAnnotationPub {
        ReviewAnnotationPub {
            id: review.id,
            user_id: review.user_id,
            created: review.created.timestamp_millis(),
            text: "".to_string,
            stars: 5,
            pic_id: review.pic_id.clone(),
            pic_url: PicPub::get_url(&review.pic_id, review.pic_handler),
            category: review.category,
            location_name: review.location_name,
            latitude: review.latitude,
            longitude: review.longitude,
            is_custom: review.is_custom,
        }
    }
}
