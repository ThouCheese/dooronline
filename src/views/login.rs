use maud::{html, Markup, DOCTYPE};
use models::User;
use rocket::{response::Redirect, request::Form, http::{Cookies, Cookie}, };
use schema::user;
use db::get_connection;
use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};

#[derive(FromForm)]
struct LoginData {
    username: String,
    password: String,
}

#[get("/login")]
fn get() -> Markup {
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

#[post("/login", data="<form>")]
fn post(form: Form<LoginData>, mut cookies: Cookies) -> Result<Redirect, Markup> {
    let form = form.get();
    user::table
        .filter(user::username.eq(&form.username.to_lowercase()))
        .get_result(&get_connection())
        .map_err(|_| ("Wow wie is dat uberhaupt?", 
                      form.username.as_str(), 
                      form.password.as_str()))
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