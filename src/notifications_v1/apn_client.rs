use chrono::prelude::*;
use jwt::{mint_apn_jwt, APNSigningKey};
use reqwest::Client;
use serde::Serialize;
use std::sync::Mutex;

use super::NotificationType;

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

    pub async fn send_notification(
        &self,
        device_token: &str,
        message: &str,
        notification_type: NotificationType,
        notification_value: Option<String>,
    ) -> Result<(), String> {
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

        let pn = PushNotification {
            aps: Alert {
                alert: message.to_string(),
                notification_type: notification_type.to_string(),
                notification_value: notification_value.unwrap_or_default(),
            },
        };

        let body: String;
        let body_res = serde_json::to_string(&pn);

        if let Ok(body_ok) = body_res {
            body = body_ok;
        } else {
            return Ok(());
        }

        let result = self
            .client
            .post(format!(
                "https://api.push.apple.com/3/device/{}",
                device_token
            ))
            .header("authorization", format!("bearer {}", &token))
            .header("apns-push-type", "alert")
            .header("apns-expiration", "0")
            .header("apns-id", "eabeae54-14a8-11e5-b60b-1697f925ec7b")
            .header("apns-topic", "com.spacedoglabs.spotster")
            .header("apns-priority", "10")
            .body(body)
            .send()
            .await;

        match result {
            Ok(resp) => {
                if resp.status().is_success() {
                    return Ok(());
                } else {
                    let mut text = "".to_string();
                    let status_code = resp.status();

                    if let Ok(text_tmp) = resp.text().await {
                        text = text_tmp.to_string();
                    }

                    let error_text =
                        format!("APN FAILED WITH STATUS {} AND BODY {}", status_code, text);

                    return Err(error_text);
                }
            }
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }
}

#[derive(Serialize)]
struct PushNotification {
    pub aps: Alert,
}

#[derive(Serialize)]
struct Alert {
    pub alert: String,
    pub notification_type: String,
    pub notification_value: String,
}
