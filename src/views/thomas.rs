use maud::{html, Markup, DOCTYPE};

#[get("/thomas/<message>")]
fn get(message: String) -> Markup {
    let message = match message.as_str() {
        "delete" => "mijn account moet verwijderen thomas!",
        "admin" => "mijn adminrechten mag verwijderen thomas!",
        _ => "wat voor shit je ook aan het doen bent doen.",
    };

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
                    a href="/login" { "Login" }
                    a href="/logout" { "Logout" }
                }
            }
            hr;
            div class="main" {
                h1 { "Thomas godverdomme" }
                p { "Wat doe je nou, ik heb nog zo gezegd dat je niet " (message) }
            }
        }
    }
}
 