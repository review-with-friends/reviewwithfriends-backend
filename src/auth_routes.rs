use crate::DBClient;
use rocket_db_pools::Connection;
use validation;

#[post("/requestcode?<phone>")]
pub async fn request_code(mut client: Connection<DBClient>, phone: &str) -> Result<(), String> {
    validation::validate_phone(phone)?;
    Ok(())
}

#[post("/signin?<code>")]
pub async fn sign_in(mut client: Connection<DBClient>, code: &str) -> Result<(), String> {
    validation::validate_code(code)?;
    Ok(())
}
