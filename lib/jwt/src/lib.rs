use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
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

/// Wrapper to ensure the APN Signing key is always loaded for sending push notifications.
#[derive(Clone)]
pub struct APNSigningKey(pub EncodingKey);

#[derive(Debug, Serialize, Deserialize)]
struct APNClaims {
    iss: String,
    iat: usize,
}

pub fn encode_apn_jwt_secret(jwt_secret: &str) -> APNSigningKey {
    let bytes = general_purpose::STANDARD.decode(jwt_secret).unwrap();
    APNSigningKey(EncodingKey::from_ec_der(&bytes))
}

pub fn mint_apn_jwt(keys: &APNSigningKey) -> String {
    const KEY_ID: &str = "63HSBC6B65";
    const TEAM_ID: &str = "W3CBQB54QR";

    let claims = APNClaims {
        iss: TEAM_ID.to_string(),
        iat: Utc::now().timestamp() as usize,
    };

    let mut header = Header::new(Algorithm::ES256);
    header.kid = Some(KEY_ID.to_string());

    encode(&header, &claims, &keys.0).unwrap()
}
