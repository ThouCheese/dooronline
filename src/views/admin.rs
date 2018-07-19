use maud::{html, Markup, DOCTYPE};
use rocket::{response::{Failure, Redirect, }, request::Form, http::Status };
use db::get_connection;
use diesel::{RunQueryDsl, insert_into, update, delete};
use models::{User, Admin, LogEntry, };
use crypto::hash_password;
use schema::{user, log};
use diesel::{QueryDsl, ExpressionMethods, };

#[derive(FromForm, Insertable)]
#[table_name = "user"]
struct NewData {
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(FromForm)]
struct DeleteForm {
    pub id: i32,
}

#[get("/admin")]
fn get(_user: Admin) -> Result<Markup, Failure> {
    let user_vector: Vec<User> = user::table.get_results(&get_connection())
        .or(Err(Failure(Status::InternalServerError)))?;
    Ok(html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { "Admin" }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/static/style.min.css";
            link rel="stylesheet" href="/static/admin-style.css";
        }
        body {
            div class="header" {
                a class="header-left" href="/" {
                    h1 { "Headeur" }
                }
                div style="float: right; padding-top: 30px;" {
                    a href="/log" { "Log" }
                    a style="margin-left: 10px;" href="/logout" { "Logout" }
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
                @for user in user_vector {
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
    })
}

#[post("/admin/edituser", data = "<form>")]
fn edit_user(user: Admin, form: Form<User>) -> Result<Redirect, Failure> {
    let mut model = form.into_inner();
    if user.id != 1 && model.id == 1 {
        return Ok(Redirect::to("/thomas/admin"));
    }
    model.username = model.username.to_lowercase();
    model.password = if model.password == "" {
        user::table
            .find(model.id)
            .get_result::<User>(&get_connection())
            .or(Err(Failure(Status::InternalServerError)))?
            .password
    } else {
        hash_password(&model.password)
            .or(Err(Failure(Status::InternalServerError)))?
    };

    update(user::table.find(model.id))
        .set((user::username.eq(model.username),
              user::password.eq(model.password),
              user::is_admin.eq(model.is_admin), ))
        .execute(&get_connection())
        .or(Err(Failure(Status::InternalServerError)))?;
    Ok(Redirect::to("/admin"))
}

#[post("/admin/deleteuser", data = "<form>")]
fn delete_user(user: Admin, form: Form<DeleteForm>) -> Result<Redirect, Failure> {
    let user_id = form.into_inner().id;
    if user.id != 1 && user_id == 1 {
        return Ok(Redirect::to("/thomas/delete"));
    }
    delete(user::table.find(user_id))
        .execute(&get_connection())
        .or(Err(Failure(Status::InternalServerError)))?;
    Ok(Redirect::to("/admin"))
}

#[post("/admin/adduser", data = "<form>")]
fn add_user(_user: Admin, form: Form<NewData>) -> Result<Redirect, Failure> {
    let mut data = form.into_inner();
    assert_ne!(data.password, "");
    data.password = hash_password(&data.password)
        .or(Err(Failure(Status::InternalServerError)))?;
    insert_into(user::table)
        .values(&data)
        .execute(&get_connection())
        .or(Err(Failure(Status::InternalServerError)))?;
    Ok(Redirect::to("/admin"))
}

#[get("/log")]
fn log(_user: Admin) -> Result<Markup, Failure> {
    let log_entries: Vec<(LogEntry, User)> = log::table
        .inner_join(user::table)
        .order(log::date.desc())
        .load(&get_connection())
        .or(Err(Failure(Status::InternalServerError)))?;
    Ok(html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { "Admin" }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/static/style.min.css";
            link rel="stylesheet" href="/static/admin-style.css";
        }
        body {
            div class="header" {
                a class="header-left" href="/" {
                    h1 { "Headeur" }
                }
                div style="float: right; padding-top: 30px;" {
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