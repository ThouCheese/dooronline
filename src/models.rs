use crypto::{verify_password, create_auth_token, validate_user_token};
use schema::{user, log};
use rocket::{http::Status, Outcome};
use rocket::request::{self, FromRequest, Request};
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Serialize, FromForm)]
#[table_name="user"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

impl User {
    pub fn validate_password(&self, password: &str) -> bool {
        match verify_password(&self.password, password) {
            Ok(is_valid) => is_valid,
            Err(_err) => {
                println!("de error is {:?}", _err);
                false
            },
        }
    }
    
    pub fn create_jwt(&self) -> Result<String, String> {
        create_auth_token(self)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, String> {
        request
            .cookies()
            .get_private("Authorization")
            .map(|cookie| String::from(cookie.value()))
            .map(|jwt| if let Some(user) = validate_user_token(&jwt) {
                Outcome::Success(user)
            } else {
                Outcome::Failure((Status::Unauthorized, String::from("invalid JWT")))
            })
            .unwrap_or(Outcome::Failure((Status::Unauthorized, String::from("invalid JWT"))))
    }
}

#[derive(Queryable)] 
// #[table_name="log"]
pub struct LogEntry {
    pub id: i32,
    pub user_id: i32,
    pub date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="log"]
pub struct NewLogEntry {
    pub user_id: i32,
    pub date: NaiveDateTime,
}
