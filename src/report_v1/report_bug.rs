use crate::{authorization::AuthenticatedUser, report_v1::GithubClient};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post,
    web::{Data, Json, ReqData},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BugReportRequest {
    title: String,
    description: String,
}

/// Report a bug.
#[post("/bug")]
pub async fn report_bug(
    _authenticated_user: ReqData<AuthenticatedUser>,
    gh_client: Data<GithubClient>,
    bug_report_request: Json<BugReportRequest>,
) -> Result<impl Responder> {
    if bug_report_request.title.chars().count() > 64 {
        return Err(ErrorBadRequest("title too long".to_string()));
    }

    if bug_report_request.title.chars().count() > 1024 {
        return Err(ErrorBadRequest("description too long".to_string()));
    }

    let report_res = gh_client
        .submit_bug(&bug_report_request.title, &bug_report_request.description)
        .await;

    match report_res {
        Ok(_) => {
            return Ok(HttpResponse::Ok().finish());
        }
        Err(_) => {
            return Err(ErrorInternalServerError(
                "unable to create bug report".to_string(),
            ));
        }
    }
}
