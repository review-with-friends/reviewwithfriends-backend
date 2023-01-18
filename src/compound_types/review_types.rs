use crate::likes_v1::LikePub;
use crate::pic_v1::PicPub;
use crate::reply_v1::ReplyPub;
use crate::review_v1::ReviewPub;
use serde::Serialize;

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
///
/// This compound pub consists of other pubs.
#[derive(Serialize)]
pub struct CompoundReviewPub {
    pub review: ReviewPub,
    pub likes: Vec<LikePub>,
    pub replies: Vec<ReplyPub>,
    pub pics: Vec<PicPub>,
}
