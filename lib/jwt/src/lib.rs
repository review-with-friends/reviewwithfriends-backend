use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub struct SigningKeys(pub EncodingKey, pub DecodingKey);

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

pub fn mint_jwt(keys: &SigningKeys, id: &str) -> String {
    let claims = Claims {
        aud: "mob".to_string(),
        // 28 day expiry
        exp: Utc::now().timestamp() as usize + 2419200 as usize,
        sub: id.to_string(),
    };

    encode(&Header::default(), &claims, &keys.0).unwrap()
}

pub fn encode_jwt_secret(jwt_secret: &str) -> SigningKeys {
    SigningKeys(
        EncodingKey::from_secret(jwt_secret.as_ref()),
        DecodingKey::from_secret(jwt_secret.as_ref()),
    )
}

/// Returns true if `key` is a valid API key string.
pub fn validate_jwt(keys: &SigningKeys, token: &str) -> Option<String> {
    match decode::<Claims>(&token, &keys.1, &Validation::default()) {
        Ok(t) => Some(t.claims.sub),
        Err(_) => None,
    }
}
