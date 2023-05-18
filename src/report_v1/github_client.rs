use reqwest::Client;
use serde::Serialize;

/// Client use to communicate with Apple Push-Notification Network.
pub struct GithubClient {
    pub client: Client,
    pub token: String,
}

impl GithubClient {
    pub async fn submit_bug(&self, title: &str, description: &str) -> Result<(), String> {
        let ghi = GithubIssue {
            title: title.to_string(),
            body: description.to_string(),
            assignee: "colathro".to_string(),
            labels: vec!["bug".to_string()],
        };

        let body: String;
        let body_res = serde_json::to_string(&ghi);

        if let Ok(body_ok) = body_res {
            body = body_ok;
        } else {
            return Ok(());
        }

        let result = self
            .client
            .post("https://api.github.com/repos/review-with-friends/reviewwithfriends-ios/issues")
            .header("authorization", format!("token {}", &self.token))
            .header("accept", "application/vnd.github+json")
            .header("user-agent", "RWF Backend")
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

                    let error_text = format!(
                        "GITHUB ISSUE CREATE FAILED WITH STATUS {} AND BODY {}",
                        status_code, text
                    );

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
struct GithubIssue {
    title: String,
    body: String,
    assignee: String,
    labels: Vec<String>,
}
