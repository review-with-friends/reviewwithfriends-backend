use async_trait::async_trait;
pub use rusoto_core::{ByteStream, HttpClient};
use rusoto_credential::{AwsCredentials, ProvideAwsCredentials};
pub use rusoto_s3::*;
use rusoto_signature::Region;

pub const DEFAULT_PIC_ID: &str = "default";

/// Build S3 Client for managing Spaces resources
pub fn create_s3_client(key: &str, secret: &str) -> S3Client {
    S3Client::new_with(
        HttpClient::new().unwrap(),
        DOCredentials {
            key: String::from(key),
            secret: String::from(secret),
        },
        DOCredentials::get_region(),
    )
}

/// Digital Ocean Credentials for Spaces
pub struct DOCredentials {
    key: String,
    secret: String,
}

impl DOCredentials {
    pub fn get_region() -> Region {
        Region::Custom {
            name: "sfo3".to_string(),
            endpoint: "sfo3.digitaloceanspaces.com".to_string(),
        }
    }
}

#[async_trait]
impl ProvideAwsCredentials for DOCredentials {
    async fn credentials(
        &self,
    ) -> Result<rusoto_credential::AwsCredentials, rusoto_credential::CredentialsError> {
        Ok(AwsCredentials::new(&self.key, &self.secret, None, None))
    }
}
