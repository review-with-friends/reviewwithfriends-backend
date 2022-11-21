use images::{DeleteObjectRequest, S3Client, DEFAULT_PIC_ID, S3};
use sqlx::MySqlPool;

use crate::db::delete_pic;

pub async fn best_effort_delete_pic(s3_client: &S3Client, pool: &MySqlPool, pic_id: &str) {
    if pic_id == DEFAULT_PIC_ID {
        return; // dont cleanup our default image :D
    }

    if let Ok(delete_pic_storage) = s3_client
        .delete_object(DeleteObjectRequest {
            bucket: "bout".to_string(),
            key: pic_id.to_string(),
            ..Default::default()
        })
        .await
    {
        let delete_pic_res = delete_pic(pool, pic_id).await;
    }
}
