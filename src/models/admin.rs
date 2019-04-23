use crate::models::user::User;
use crate::schema::user;
use rocket::request::{self, FromRequest, Request};
use rocket::{http::Status, Outcome};
use serde::Serialize;

#[derive(Queryable, Identifiable, Serialize, FromForm, AsChangeset, Insertable)]
#[table_name = "user"]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

impl Admin {
    // pub fn validate_password(&self, password: &str) -> bool {
    //     match verify_password(&self.password, password) {
    //         Ok(is_valid) => is_valid,
    //         Err(_err) => {
    //             println!("de error is {:?}", _err);
    //             false
    //         }
    //     }
    // }

    // fn as_user(&self) -> User {
    //     User {
    //         id: self.id,
    //         username: self.username.clone(),
    //         password: self.password.clone(),
    //         is_admin: true,
    //     }
    // }

    // pub fn create_jwt(&self) -> Result<String, String> {
    //     create_auth_token(&self.as_user())
    // }

    // pub fn all(conn: &PgConnection) -> Vec<User> {
    //     user::table.get_results(conn).unwrap()
    // }

    // pub fn by_id(id: i32, conn: &PgConnection) -> Option<Admin> {
    //     user::table.find(id).get_result(conn).ok()
    // }

    // pub fn update(&self, conn: &PgConnection) -> Option<Admin> {
    //     dsl::insert_into(user::table)
    //         .values(self)
    //         .on_conflict(user::id)
    //         .do_update()
    //         .set(self)
    //         .get_result(conn)
    //         .ok()
    // }
}

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Admin, String> {
        let user = User::from_request(request)?;
        if user.is_admin {
            Outcome::Success(user.as_admin().unwrap())
        } else {
            Outcome::Failure((Status::Unauthorized, String::from("invalid JWT")))
        }
    }
}
