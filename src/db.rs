use diesel::Connection;

#[database("door")]
pub struct DeurDB(diesel::PgConnection);

pub fn sync_connection() -> diesel::PgConnection {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    diesel::PgConnection::establish(&database_url).expect("Database not working")
}
