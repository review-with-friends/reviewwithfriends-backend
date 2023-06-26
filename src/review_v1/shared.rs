use crate::{
    compound_types::CompoundReviewPub,
    db::{does_bookmark_exist, get_all_likes, get_all_pics, get_all_replies},
    likes_v1::LikePub,
    pic_v1::PicPub,
    reply_v1::ReplyPub,
};
use sqlx::{Error, MySqlPool};

use super::ReviewPub;

pub async fn gather_compound_review(
    pool: &MySqlPool,
    review: ReviewPub,
) -> Result<CompoundReviewPub, Error> {
    let likes_pub: Vec<LikePub>;
    let likes = get_all_likes(&pool, &review.id).await?;
    likes_pub = likes.into_iter().map(|f| -> LikePub { f.into() }).collect();

    let replies_pub: Vec<ReplyPub>;
    let replies = get_all_replies(&pool, &review.id).await?;
    replies_pub = replies
        .into_iter()
        .map(|f| -> ReplyPub { f.into() })
        .collect();

    let pics_pub: Vec<PicPub>;
    let pics = get_all_pics(&pool, &review.id).await?;
    pics_pub = pics.into_iter().map(|f| -> PicPub { f.into() }).collect();

    let is_bookmarked = does_bookmark_exist(
        &pool,
        &review.user_id,
        &review.location_name,
        review.latitude,
        review.longitude,
    )
    .await?;

    return Ok(CompoundReviewPub {
        review,
        bookmarked: is_bookmarked,
        likes: likes_pub,
        replies: replies_pub,
        pics: pics_pub,
    });
}
