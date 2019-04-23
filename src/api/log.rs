use crate::db;
use crate::models::{admin::Admin, log_entry::LogEntry};
use rocket_contrib::json::Json;

#[get("/")]
pub fn list(_user: Admin, conn: db::DeurDB) -> Json<Vec<LogEntry>> {
    Json(LogEntry::all(&conn))
}
