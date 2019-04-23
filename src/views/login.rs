use crate::db;
use crate::models::user::User;
use maud::{html, Markup, DOCTYPE};
use rocket::{
    http::{Cookie, Cookies},
    request::Form,
    response::Redirect,
};

#[derive(FromForm)]
pub struct LoginData {
    username: String,
    password: String,
}

#[get("/login")]
pub fn get() -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { "Login" }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/static/style.min.css";
        }
        body {
            div class="header" {
                a class="header-left" href="/" {
                    h1 { "Headeur" }
                }
            }
            hr;
            div class="main" {
                form action="/login" method="post" class="login-form" {
                    input name="username" placeholder="username" type="text";
                    input name="password" placeholder="password" type="password";
                    input style="width: 410px;" type="submit" value="login";
                }
            }
        }
    }
}

#[post("/login", data = "<form>")]
pub fn post(
    form: Form<LoginData>,
    mut cookies: Cookies,
    conn: db::DeurDB,
) -> Result<Redirect, Markup> {
    let form = form.into_inner();
    User::by_username(&form.username, &conn)
        .ok_or_else(|| {
            (
                "Wow wie is dat uberhaupt?",
                form.username.as_str(),
                form.password.as_str(),
            )
        })
        .and_then(|user: User| {
            if user.validate_password(&form.password) {
                user.create_jwt()
                    .map_err(|_| ("Geen logintoken voor jou haha", "", ""))
                    .map(|jwt| Cookie::new("Authorization", jwt))
            } else {
                Err(("Je wachtwoord is kut en fout", form.username.as_str(), ""))
            }
        })
        .and_then(|mut cookie| {
            cookie.make_permanent();
            cookies.add_private(cookie);
            Ok(Redirect::to("/"))
        })
        .map_err(|(message, username, password)| {
            html! {
                (DOCTYPE)
                head {
                    meta charset="UTF-8";
                    title { "Login" }
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    link rel="stylesheet" href="/static/style.min.css";
                }
                body {
                    div class="header" {
                        a class="header-left" href="/" {
                            h1 { "Headeur" }
                        }
                    }
                    hr;
                    div class="failed" {
                        p { (message) }
                    }
                    div class="main" {
                        form action="/login" method="post" class="login-form" {
                            input name="username" placeholder="username" type="text"
                                  value=(username);
                            input name="password" placeholder="password" type="password"
                                  value=(password);
                            input style="width: 410px;" type="submit" value="login";
                        }
                    }
                }
            }
        })
}
