use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Wrapper to ensure EncodingKey and DecodingKeys are persisted for the duration of the service.
#[derive(Clone)]
pub struct SigningKeys(pub EncodingKey, pub DecodingKey);

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

/// Create a JWT with the given signing keys
pub fn mint_jwt(keys: &SigningKeys, id: &str) -> String {
    let claims = Claims {
        aud: "mob".to_string(),
        exp: Utc::now().timestamp() as usize + 432000000 as usize,
        sub: id.to_string(),
    };

    encode(&Header::default(), &claims, &keys.0).unwrap()
}

/// Initialize the shared SigningKeys reference for token minting and validation.
pub fn encode_jwt_secret(jwt_secret: &str) -> SigningKeys {
    SigningKeys(
        EncodingKey::from_secret(jwt_secret.as_ref()),
        DecodingKey::from_secret(jwt_secret.as_ref()),
    )
}

/// Returns 'string' sub claim if `token` is a valid
pub fn validate_jwt(keys: &SigningKeys, token: &str) -> Option<String> {
    match decode::<Claims>(&token, &keys.1, &Validation::default()) {
        Ok(t) => Some(t.claims.sub),
        Err(_) => None,
    }
}
