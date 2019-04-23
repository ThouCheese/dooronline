use crate::db;
use crate::models::user::User;
use crate::views::index::open_door;
use rocket::http::Status;

#[get("/open")]
pub fn open(user: User, conn: db::DeurDB) -> Result<(), Status> {
    open_door(&user, conn)
}
