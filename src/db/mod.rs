use rocket_db_pools::{sqlx, Database};

pub mod types;
pub use types::*;

pub mod query;
pub use query::*;

pub mod execute;
pub use execute::*;

#[derive(Database)]
#[database("mob")]
pub struct DBClient(sqlx::MySqlPool);
