use maud::{html, Markup, DOCTYPE};

#[catch(400)]
fn bad_request() -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { "400" }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/static/style.min.css";
        }
        body {
            div class="header" {
                a class="header-left" href="/" {
                    h1 { "Headeur" }
                }
            }
            div class="main" {
                h1 { "400 Bad Request, stop fucking with the forms!" }
                p { 
                    "Ik weet dat jij dit bent Thomas, en hier is mijn" 
                    a href="/ik-ben-een-flapdrol-die-met-forms-fuckt" {
                        "wraak"
                    }
                }
            }
        }
    }
}

#[catch(401)]
fn unauthorized() -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { "401" }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/static/style.min.css";
        }
        body {
            div class="header" {
                a class="header-left" href="/" {
                    h1 { "Headeur" }
                }
                div class="header-right" {
                    a href="/login" { "Log In" }
                }
            }
            hr;
            div class="main" {
                h1 { "401 Unauthorized" }
                h2 { "You fucked with the wrong site!" }
                p style="width: 100%; margin-left: auto; margin-right: auto;" {"
What the fuck did you just fucking say about me, you little bitch? I'll have you know I 
graduated top of my class in the Navy Seals, and I've been involved in numerous secret raids 
on Al-Quaeda, and I have over 300 confirmed kills. I am trained in gorilla warfare and I'm 
the top sniper in the entire US armed forces. You are nothing to me but just another target. 
I will wipe you the fuck out with precision the likes of which has never been seen before on 
this Earth, mark my fucking words. You think you can get away with saying that shit to me 
over the Internet? Think again, fucker. As we speak I am contacting my secret network of 
spies across the USA and your IP is being traced right now so you better prepare for the 
storm, maggot. The storm that wipes out the pathetic little thing you call your life. You're 
fucking dead, kid. I can be anywhere, anytime, and I can kill you in over seven hundred ways, 
and that's just with my bare hands. Not only am I extensively trained in unarmed combat, but 
I have access to the entire arsenal of the United States Marine Corps and I will use it to 
its full extent to wipe your miserable ass off the face of the continent, you little shit. If
only you could have known what unholy retribution your little \"clever\" comment was about to
bring down upon you, maybe you would have held your fucking tongue. But you couldn't, you 
didn't, and now you're paying the price, you goddamn idiot. I will shit fury all over you and 
you will drown in it. You're fucking dead, kiddo.
"
                }
            }
        }
    }
}

#[catch(403)]
fn forbidden() -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { "403" }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/static/style.min.css";
        }
        body {
            div class="header" {
                a class="header-left" href="/" {
                    h1 { "Headeur" }
                }
                div class="header-right" {
                    a href="/logout" { "Logout" }
                }
            }
            div class="main" {
                h1 { "403 Forbidden" }
                p { "
Admin rechten verkrijgen.... <FAILURE> klaarblijkelijk mag je niks HAHAHA zoek zelf een site
"
                }
            }
        }
    }
}

#[catch(404)]
fn not_found() -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { "404" }
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
                h1 { "404 Not Found" }
                p { "
Hoi! Deze pagina bestaat niet! Waar kijk ik nu dan naar zul je je afvragen... Nou dit is een
handler die dit soort dingen afvangt. Ik heb bij mezelf overwogen wat meer werk zou zijn: 
letterlijk elke pagina zinnige content geven of 1 zo'n handler schrijven. Ik koos voor
dittem. 
Kijk het werkt:"    
                }
                ul {
                    li { a href="/asdfasdfasdf" { "Dit gaat nergens heen" } }
                    li { a href="/pannekoek" { "Dit gaat nergens heen" } }
                    li { a href="/epische-megazorvens" { "Dit gaat nergens heen" } }
                    li { a href="/404" { "Dit gaat nergens heen" } }
                    li { a href="/spoiler-alert!-ook-niks" { "Dit gaat nergens heen" } }
                }
            }
        }
    }
}

#[catch(422)]
fn unprocessable() -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { "422" }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/static/style.min.css";
        }
        body {
            div class="header" {
                a class="header-left" href="/" {
                    h1 { "Headeur" }
                }
            }
            div class="main" {
                h1 { "400 Bad Request, stop fucking with the forms!" }
                p { 
                    "Ik weet dat jij dit bent Thomas, en hier is mijn" 
                    a href="/ik-ben-een-flapdrol-die-met-forms-fuckt" {
                        "wraak"
                    }
                }
                p { 
                    "Voetnoot: je had eigenlijk een 421, maar potaaaato potaaaaahto"
                }
            }
        }
    }
}

#[catch(500)]
fn internal() -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            title { "500" }
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
                h1 { "500 Internal Server Error" }
                p { "
De fout zit binnenin de server.... Maar waarschijnlijk is het toch jouw schuld. Beter ga je 
nu fixen dat je geen errors meer krijgt. Voor verdere vragen, bel Remco Jelsma,
eindverantwoordelijke van deze website, op zijn persoonlijke telefoonnummer, 
+31 6 14 10 65 71, en val je hem zo uitvoerig mogelijk lastig met nare opmerkingen en vragen 
over al dat wat hij fout heeft gedaan aan deze site."
                }
            }
        }
    }
}
