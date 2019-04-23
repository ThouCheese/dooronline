use crate::crypto::{create_auth_token, hash_password, validate_user_token, verify_password};
use crate::models::admin::Admin;
use crate::schema::user;
use diesel::{self as dsl, prelude::*};
use rocket::request::{self, FromRequest, Request};
use rocket::{http::Status, Outcome};
use serde::Serialize;

#[derive(Queryable, Identifiable, Serialize, FromForm, AsChangeset, Insertable)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub is_admin: bool,
}

#[derive(Insertable)]
#[table_name = "user"]
struct NewUser {
    username: String,
    password: String,
    is_admin: bool,
}

impl User {
    pub fn validate_password(&self, password: &str) -> bool {
        match verify_password(&self.password, password) {
            Ok(is_valid) => is_valid,
            Err(_err) => false,
        }
    }

    pub fn create(
        username: &str,
        password: &str,
        is_admin: bool,
        conn: &PgConnection,
    ) -> Option<User> {
        let new_user = NewUser {
            username: username.to_string(),
            password: hash_password(password),
            is_admin,
        };

        dsl::insert_into(user::table)
            .values(&new_user)
            .get_result(conn)
            .ok()
    }

    pub fn create_jwt(&self) -> Result<String, String> {
        create_auth_token(self)
    }

    pub fn all(conn: &PgConnection) -> Vec<User> {
        user::table.get_results(conn).unwrap()
    }

    pub fn by_id(id: i32, conn: &PgConnection) -> Option<User> {
        user::table.find(id).get_result(conn).ok()
    }

    pub fn update(&self, conn: &PgConnection) -> Option<User> {
        dsl::insert_into(user::table)
            .values(self)
            .on_conflict(user::id)
            .do_update()
            .set(self)
            .get_result(conn)
            .ok()
    }

    pub fn by_username(username: &str, conn: &PgConnection) -> Option<User> {
        user::table
            .filter(user::username.eq(&username.to_lowercase()))
            .get_result(conn)
            .ok()
    }

    pub fn as_admin(&self) -> Option<Admin> {
        if self.is_admin {
            Some(Admin {
                id: self.id,
                username: self.username.clone(),
                password: self.password.clone(),
                is_admin: true,
            })
        } else {
            None
        }
    }

    pub fn delete(self, conn: &PgConnection) -> Option<()> {
        dsl::delete(user::table.find(self.id)).execute(conn).ok()?;
        Some(())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, String> {
        let cookie = request.cookies().get_private("Authorization");
        let header = request.headers().get("Authorization").next();
        let jwt = match (cookie, header) {
            (Some(cookie), _) => cookie.value().to_string(),
            (_, Some(header)) => header.to_string(),
            _ => return Outcome::Failure((Status::Unauthorized, String::from("no JWT"))),
        };
        if let Some(user) = validate_user_token(&jwt) {
            Outcome::Success(user)
        } else {
            Outcome::Failure((Status::Unauthorized, String::from("invalid JWT")))
        }
    }
}
