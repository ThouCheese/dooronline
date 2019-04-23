use crate::crypto::hash_password;
use crate::db;
use crate::models::{admin::Admin, log_entry::LogEntry, user::User};
use maud::{html, Markup, DOCTYPE};
use rocket::{http::Status, request::Form, response::Redirect};

#[derive(FromForm)]
pub struct NewData {
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(FromForm)]
pub struct DeleteForm {
    pub id: i32,
}

#[get("/admin")]
pub fn get(_user: Admin, conn: db::DeurDB) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { "Admin" }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/static/style.min.css";
            link rel="stylesheet" href="/static/admin-style.min.css";
        }
        body {
            div class="header" {
                a class="header-left" href="/" {
                    h1 { "Headeur" }
                }
                div class="header-right" {
                    a href="/log" { "Log" }
                    a href="/logout" { "Logout" }
                }
            }
            hr;
            div class="main" {
                h2 { "New User" }
                div class="add-user" {
                    form action="/admin/adduser" method="post" {
                        input name="username" placeholder="username";
                        input name="password" placeholder="password";
                        "Admin?"
                        input name="is_admin" type="checkbox";
                        input type="submit" value="create";
                    }
                }

                h2 { "User list" }
                @for user in User::all(&conn) {
                    div class="user-list" {
                        form action="/admin/edituser" method="post" class="update-form" {
                            p { (user.id) }
                            input type="hidden" name="id" value=(user.id);
                            input type="text" name="username" value=(user.username)
                                  class="username";
                            input type="text" name="password" placeholder="password"
                                  autocomplete="off";
                            p { "admin?" }
                            input type="checkbox" name="is_admin" checked?[user.is_admin];
                            input type="submit" value="update user";
                        }
                        form action="/admin/deleteuser" method="post" class="delete-form" {
                            input type="hidden" name="id" value=(user.username);
                            input type="submit" value="delete user";
                        }
                    }
                    hr;
                }
            }
        }
    }
}

#[post("/admin/edituser", data = "<form>")]
pub fn edit_user(user: Admin, form: Form<User>, conn: db::DeurDB) -> Result<Redirect, Status> {
    let form = form.into_inner();
    if user.id != 1 && form.id == 1 {
        return Ok(Redirect::to("/thomas/admin"));
    }
    let mut updatee = User::by_id(form.id, &conn).ok_or_else(|| Status::InternalServerError)?;
    updatee.username = form.username.to_lowercase();
    updatee.password = if form.password == "" {
        updatee.password
    } else {
        hash_password(&form.password)
    };
    updatee.update(&conn).ok_or(Status::InternalServerError)?;

    Ok(Redirect::to("/admin"))
}

#[post("/admin/deleteuser", data = "<form>")]
pub fn delete_user(
    user: Admin,
    form: Form<DeleteForm>,
    conn: db::DeurDB,
) -> Result<Redirect, Status> {
    let user_id = form.into_inner().id;
    if user.id != 1 && user_id == 1 {
        return Ok(Redirect::to("/thomas/delete"));
    }
    let user = User::by_id(user_id, &conn).unwrap();
    user.delete(&conn).ok_or(Status::InternalServerError)?;
    Ok(Redirect::to("/admin"))
}

#[post("/admin/adduser", data = "<form>")]
pub fn add_user(_user: Admin, form: Form<NewData>, conn: db::DeurDB) -> Result<Redirect, Status> {
    let form = form.into_inner();
    assert_ne!(form.password, "");
    User::create(&form.username, &form.password, form.is_admin, &conn)
        .ok_or(Status::InternalServerError)?;
    Ok(Redirect::to("/admin"))
}

#[get("/log")]
pub fn log(_user: Admin, conn: db::DeurDB) -> Result<Markup, Status> {
    let log_entries = LogEntry::all_with_user(&conn);
    Ok(html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { "Admin" }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/static/style.min.css";
            link rel="stylesheet" href="/static/admin-style.min.css";
        }
        body {
            div class="header" {
                a class="header-left" href="/" {
                    h1 { "Headeur" }
                }
                div class="header-right" {
                    a href="/admin" { "Admin" }
                    a href="/logout" { "Logout" }
                }
            }
            hr;
            div class="main" {
                h2 { "Log entries" }
                div class="log-entry" {
                    p class="log-entry-element" { i { "ID" } }
                    p class="log-entry-element" { i { "User" } }
                    p class="date-header" { i { "Date" } }
                }
                @for (entry, user) in log_entries {
                    div class="log-entry" {
                        p class="log-entry-element" { (entry.id) }
                        p class="log-entry-element" { (user.username) }
                        p class="date" { (entry.date.format("%Y-%m-%d %H:%M:%S")) }
                        // p class="date" { (entry.date.format("%d-%m-%Y %H:%M:%S")) }
                    }
                    hr;
                }
            }
        }
    })
}
