use crate::models::user::User;
use maud::{html, Markup, DOCTYPE};
use rocket::{
    http::{Cookie, Cookies},
    response::Redirect,
};

#[get("/logout")]
pub fn get() -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { "Logout" }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/static/style.min.css";
        }
        body {
            div class="header" {
                a class="header-left" href="/" {
                    h1 { "Headeur" }
                }
                div class="header-right" {
                    a href="/admin" { "Admin" }
                }
            }
            hr;
            div class="main" {
                h1 { "NoOOOoOOooOo!!!!!1!" }
                p {
                    "
Ga je weg? Waarom? Waarheen? Is er een andere site? Dacht ik het niet! Het is zeker die sloerie
van een 216.18.168.16:80 of niet? Ik zag je wel rondhangen met haar! \"Ik moet weer laat 
werken\" elke fucking keer weer! En ik blijf het maar geloven, en waarom? Wat krijg ik er voor 
terug? Niks! Ik kan hier een beetje alleen achter blijven met de kinderen terwijl jij alle 
poorten van 216.18.168.16 zit te gebruiken!"
                }
                form action="/logout" method="post" class="login-form" {
                    input type="submit" value="logout";
                }
            }
        }
    }
}

#[post("/logout")]
pub fn post(_user: User, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("Authorization"));
    Redirect::to("/login")
}
