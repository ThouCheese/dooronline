use crate::models::user::User;
use crate::schema::{log, user};
use chrono::{NaiveDateTime, Utc};
use diesel::{self as dsl, prelude::*};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct LogEntry {
    pub id: i32,
    pub user_id: i32,
    pub date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "log"]
struct NewLogEntry {
    user_id: i32,
    date: NaiveDateTime,
}

impl LogEntry {
    pub fn create(user_id: i32, conn: &PgConnection) -> Option<LogEntry> {
        let new_log_entry = NewLogEntry {
            user_id,
            date: Utc::now().naive_local(),
        };

        dsl::insert_into(log::table)
            .values(&new_log_entry)
            .get_result(conn)
            .ok()
    }

    pub fn all_with_user(conn: &PgConnection) -> Vec<(LogEntry, User)> {
        log::table
            .inner_join(user::table)
            .order(log::date.desc())
            .get_results(conn)
            .unwrap()
    }

    pub fn all(conn: &PgConnection) -> Vec<LogEntry> {
        log::table.get_results(conn).unwrap()
    }
}
