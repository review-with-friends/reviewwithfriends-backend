use crate::db::Report;
use chrono::NaiveDateTime;
use serde::Serialize;

/// DB Types are purposefuly not serialized.
/// We require DTO objects suffixed with 'Pub'
/// to trim database object appropriately.
#[derive(Serialize)]
pub struct ReportPub {
    /// Guid unique identifier.
    pub id: String,
    /// Datetime the report was made.
    pub created: NaiveDateTime,
    /// The user who got reported
    pub user_id: String,
    // Id of the user who reported
    pub reporter_id: String,
    /// The type of report.
    pub report_type: u8,
}

impl From<Report> for ReportPub {
    fn from(report: Report) -> ReportPub {
        ReportPub {
            id: report.id,
            created: report.created,
            user_id: report.user_id,
            reporter_id: report.reporter_id,
            report_type: report.report_type,
        }
    }
}
