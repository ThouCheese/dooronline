use crate::crypto::hash_password;
use crate::db;
use crate::models::{admin::Admin, user::User};
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[post("/login", data = "<form>")]
pub fn login(form: Json<LoginForm>, conn: db::DeurDB) -> Result<Json<LoginResponse>, Status> {
    let user = User::by_username(&form.username, &conn).ok_or(Status::BadRequest)?;
    if user.validate_password(&form.password) {
        Ok(Json(LoginResponse {
            token: user.create_jwt().unwrap(),
        }))
    } else {
        Err(Status::Unauthorized)
    }
}

#[derive(Deserialize)]
pub struct CreateForm<'a> {
    username: &'a str,
    password: &'a str,
    is_admin: bool,
}

#[post("/", data = "<form>")]
pub fn create(
    form: Json<CreateForm>,
    _user: Admin,
    conn: db::DeurDB,
) -> Result<Json<User>, Status> {
    let user = User::create(form.username, form.password, form.is_admin, &conn)
        .ok_or(Status::InternalServerError)?;
    Ok(Json(user))
}

#[derive(Deserialize)]
pub struct UpdateForm<'a> {
    username: Option<&'a str>,
    password: Option<&'a str>,
    is_admin: Option<bool>,
}

#[patch("/<user_id>", data = "<form>")]
pub fn update(
    user_id: i32,
    form: Json<UpdateForm>,
    user: User,
    conn: db::DeurDB,
) -> Result<Json<User>, Status> {
    if !user.is_admin || (user.id != 1 && user_id == 1) {
        return Err(Status::Unauthorized);
    }
    let mut editee = User::by_id(user_id, &conn).ok_or(Status::BadRequest)?;
    editee.username = form.username.map(str::to_string).unwrap_or(editee.username);
    editee.password = form.password.map(hash_password).unwrap_or(editee.password);
    editee.is_admin = form.is_admin.unwrap_or(editee.is_admin);
    editee.update(&conn).ok_or(Status::InternalServerError)?;
    Ok(Json(user))
}

#[get("/", rank = 2)]
pub fn list(_user: Admin, conn: db::DeurDB) -> Json<Vec<User>> {
    Json(User::all(&conn))
}

#[get("/?<user_id>")]
pub fn get(user_id: i32, _user: Admin, conn: db::DeurDB) -> Result<Json<User>, Status> {
    Ok(Json(User::by_id(user_id, &conn).ok_or(Status::NotFound)?))
}
