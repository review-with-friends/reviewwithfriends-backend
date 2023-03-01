use crate::{authorization::AuthenticatedUser, db::phone_number_discovery, user_v1::UserPub};
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Json, ReqData},
    Responder, Result,
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use sqlx::MySqlPool;
use validation::validate_phone;

#[derive(Deserialize)]
pub struct DiscoveryRequest {
    numbers: Vec<String>,
}

/// Allows users to discover friends using the app from sharing their contacts.
#[post("/discover_friends")]
pub async fn discover_friends(
    _: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
    disco_request: Json<DiscoveryRequest>,
) -> Result<impl Responder> {
    let input_numbers: Vec<&str> = disco_request
        .numbers
        .iter()
        .filter(|number| validate_phone(number).is_ok())
        .map(|number| number as &str)
        .collect();

    // If we don't have enough numbers, don't even try to lookup.
    if input_numbers.len() == 0 {
        return Ok(Json(Vec::<UserPub>::new()));
    }

    let discovery_query_res = phone_number_discovery(&pool, &input_numbers).await;

    if let Ok(discovery_results) = discovery_query_res {
        let mut output: Vec<UserPub> = discovery_results
            .into_iter()
            .map(|f| -> UserPub { f.into() })
            .collect();

        // shuffle the output to minimize any predictability
        let mut rng = thread_rng();
        output.shuffle(&mut rng);

        // take only the first 50 after shuffling
        // in most cases, people won't have more than this
        output.truncate(50);

        return Ok(Json(output));
    } else {
        return Err(ErrorInternalServerError("failed to discover friends"));
    }
}
