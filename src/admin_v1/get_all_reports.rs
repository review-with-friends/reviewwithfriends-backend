use crate::{admin_v1::ReportPub, authorization::AuthenticatedUser, db};
use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Json, ReqData},
    Responder, Result,
};
use sqlx::MySqlPool;

/// Searches for users by name.
/// Returns a list of the results.
#[get("/all_reports")]
pub async fn get_all_reports(
    _authenticated_user: ReqData<AuthenticatedUser>,
    pool: Data<MySqlPool>,
) -> Result<impl Responder> {
    let report_res = db::get_all_reports(&pool).await;

    match report_res {
        Ok(results) => {
            let reports_pub: Vec<ReportPub> = results
                .into_iter()
                .map(|f| -> ReportPub { f.into() })
                .collect();
            Ok(Json(reports_pub))
        }
        Err(_) => return Err(ErrorInternalServerError("could not fetch reports")),
    }
}
