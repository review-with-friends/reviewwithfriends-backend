use super::{mint_jwt, SigningKeys};
use rocket::State;

#[get("/")]
pub async fn initiate_phone_auth(signing_keys: &State<SigningKeys>) -> String {
    let jwt = mint_jwt(&signing_keys, "1");
    return jwt;
}
