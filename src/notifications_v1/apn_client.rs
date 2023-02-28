use chrono::prelude::*;
use jwt::{mint_apn_jwt, APNSigningKey};
use reqwest::Client;
use std::sync::Mutex;

/// Client use to communicate with Apple Push-Notification Network.
pub struct APNClient {
    pub client: Client,
    pub key: APNSigningKey,
    pub token: Mutex<String>,
    pub issued_time: Mutex<i64>,
}

impl APNClient {
    fn generate_token(key: &APNSigningKey) -> String {
        mint_apn_jwt(key)
    }

    pub async fn send_notification(&self) -> Result<(), String> {
        let token: String;

        {
            let time_lock_res = self.issued_time.lock();
            match time_lock_res {
                Ok(mut issued_time) => {
                    let token_lock_res = self.token.lock();
                    match token_lock_res {
                        Ok(mut token_g) => {
                            if Utc::now().timestamp() - *issued_time >= 2700 {
                                let tmp_token = APNClient::generate_token(&self.key);
                                *token_g = tmp_token.clone();
                                *issued_time = Utc::now().timestamp();

                                token = tmp_token;
                            } else {
                                token = token_g.clone();
                            }
                        }
                        Err(_) => return Err("unable to get token lock".to_string()),
                    }
                }
                Err(_) => return Err("unable to get issued lock".to_string()),
            }
        }

        let result = self
            .client
            .post("https://api.sandbox.push.apple.com/3/device/3015A7F5BEA16B7A640A152C617BA52D3396DA8194AFCCF017D1BEE32EDB2AC2")
            .header("authorization", format!("bearer {}", &token))
            .header("apns-push-type", "alert")
            .header("apns-expiration", "0")
            .header("apns-id", "eabeae54-14a8-11e5-b60b-1697f925ec7b")
            .header("apns-topic", "com.spacedoglabs.spotster")
            .header("apns-priority","10")
            .body("{ \"aps\" : { \"alert\" : \"Hello\", \"badge\" : 1 } }")
            .send()
            .await;

        match result {
            Ok(resp) => {
                println!("{}", resp.status());
                println!("{}", resp.text().await.unwrap());
                return Ok(());
            }
            Err(error) => println!("{}", error),
        }

        Ok(())
    }
}
