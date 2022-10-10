use crate::{
    db::{get_user, DBClient},
    JWTAuthorized,
};

#[get("/auth_helloworld")]
pub async fn auth_hello_world(auth: JWTAuthorized, client: &DBClient) -> Result<String, String> {
    let user_req = get_user(client, auth.0).await;

    match user_req {
        Ok(user) => Ok(user.display_name.to_string()),
        Err(_) => Err("could not get user".to_string()),
    }
}
