use crate::db;
use crate::models::{log_entry::LogEntry, user::User};
use maud::{html, Markup, DOCTYPE};
use rocket::http::Status;
use std::{thread, time};
use sysfs_gpio::{Direction, Pin};

static SLEEP_TIME: time::Duration = time::Duration::from_millis(75);

#[get("/")]
pub fn get(user: User) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { "Open door" }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/static/style.min.css";
        }
        body {
            div class="header" {
                a class="header-left" href="/" {
                    h1 { "Headeur" }
                }
                div class="header-right" {
                    @if user.is_admin {
                        a href="/admin" { "Admin" }
                    }
                    a href="/logout" { "Logout" }
                }
            }
            hr;
            div class="main" {
                form action="/" method="post" class="door-form" {
                    input type="submit" value="Open da door";
                }
            }
        }
    }
}

pub fn open_door(user: &User, conn: db::DeurDB) -> Result<(), Status> {
    LogEntry::create(user.id, &conn).ok_or(Status::InternalServerError)?;

    let my_led = Pin::new(27);
    my_led
        .with_exported(|| {
            my_led.set_direction(Direction::Out)?;
            my_led.set_value(1)?;
            thread::sleep(SLEEP_TIME);
            my_led.set_value(0)?;
            Ok(())
        })
        .or(Err(Status::InternalServerError))?;
    Ok(())
}

#[post("/")]
pub fn post(user: User, conn: db::DeurDB) -> Result<Markup, Status> {
    // create log entry first, we log failed attempts as well
    open_door(&user, conn)?;

    Ok(html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { "Open door" }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/static/style.min.css";
        }
        body {
            div class="header" {
                a class="header-left" href="/" {
                    h1 { "Headeur" }
                }
                div class="header-right" {
                    @if user.is_admin {
                        a href="/admin" { "Admin" }
                    }
                    a href="/logout" { "Logout" }
                }
            }
            hr;
            div class="main" {
                form action="/" method="post" class="door-form" {
                    input type="submit" value="Open da door";
                }
            }
        }
    })
}
