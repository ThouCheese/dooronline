#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate sysfs_gpio;
#[macro_use] 
extern crate diesel;
extern crate dotenv;
extern crate argon2rs;
extern crate jsonwebtoken as jwt;
extern crate rand;
extern crate serde;
#[macro_use] 
extern crate serde_derive;
extern crate chrono;

mod models;
mod schema;
mod db;
mod crypto;
mod minify;
#[macro_use]
mod macros;

use rocket::{request::Form, 
             response::{Redirect, Failure, NamedFile, }, 
             http::{Cookie, Cookies, Status, }, };
use rocket_contrib::Template;
use models::{User, NewLogEntry, LogEntry};
use schema::{user, log};
use db::get_connection;
use std::{path::{Path, PathBuf, },
          thread, time, };
#[allow(unused_imports)] 
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, };
use sysfs_gpio::{Direction, Pin, };
use chrono::{Utc, NaiveDateTime, };
use crypto::hash_password;

static SLEEP_TIME: time::Duration = time::Duration::from_millis(75);

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    if file.to_str().or(None)?.contains("..") {
        panic!("Attempt to access root directory")
    }
    NamedFile::open(Path::new("static/").join(file)).ok()
}

mod catchers {
    use rocket_contrib::Template;
    
    #[error(400)]
    fn bad_request() -> Template {
        Template::render("400.min", &hashmap!["422" => false])
    }
    
    #[error(401)]
    fn unauthorized() -> Template {
        Template::render("401.min", &())
    }
    
    #[error(403)]
    fn forbidden() -> Template {
        Template::render("403.min", &())
    }
    
    #[error(404)]
    fn not_found() -> Template {
        Template::render("404.min", &())
    }
    
    #[error(422)]
    fn unprocessable() -> Template {
        Template::render("400.min", &hashmap!["422" => true])
    }
    
    #[error(500)]
    fn internal() -> Template {
        Template::render("500.min", &())
    }
}

mod index {
    use super::*;

    #[get("/")]
    fn get(/*user: User*/) -> Template {
        Template::render("index.min", &hashmap!["admin" => /*user.is_admin*/true])
    }

    #[post("/")]
    fn post(user: User) -> Result<Template, Failure> {
        // create log entry first, we log failed attempts as well
        let new_log_entry = NewLogEntry {
            user_id: user.id,
            date: Utc::now().naive_local(),
        };

        diesel::insert_into(schema::log::table)
            .values(&new_log_entry)
            .execute(&get_connection())
            .or(Err(Failure(Status::InternalServerError)))?;

        let my_led = Pin::new(27);
        my_led.with_exported(|| {
            my_led.set_direction(Direction::Out)?;
            my_led.set_value(1)?;
            thread::sleep(SLEEP_TIME);
            my_led.set_value(0)?;
            Ok(())
        }).or(Err(Failure(Status::InternalServerError)))?;
        Ok(Template::render("index.min", &hashmap!["admin" => user.is_admin]))
    }
}

mod login {
    use super::*;
    
    #[derive(FromForm)]
    struct LoginData {
        username: String,
        password: String,
    }

    #[get("/login")]
    fn get() -> Template {
        Template::render("login.min", &hashmap!["failed-attempt" => false])
    }

    #[post("/login", data="<form>")]
    fn post(form: Form<LoginData>, mut cookies: Cookies) -> Result<Redirect, Template> {
        let user: User = user::table
            .filter(user::username.eq(&form.get().username.to_lowercase()))
            .get_result(&db::get_connection())
            .or(Err(Template::render(
                "login.min", 
                &hashmap!["message" => "Wow wie is dat uberhaupt?"]
            )))?;
        if user.validate_password(&form.get().password) {
            let mut cookie = Cookie::new("Authorization", user.create_jwt()
                .or(Err(Template::render(
                    "login.min", 
                    &hashmap!["message" => "Geen logintoken voor jou haha"])
                ))?);
            cookie.make_permanent();
            cookies.add_private(cookie);
            Ok(Redirect::to("/"))
        } else {
            Err(Template::render(
                "login.min", 
                &hashmap!["message" => "Je wachtwoord is kut en fout"]
            ))
        }
    }
}

mod logout {
    use super::*;
    
    #[get("/logout")]
    fn get() -> Template {
        Template::render("logout.min", &()) 
    }

    #[post("/logout")]
    fn post(_user: User, mut cookies: Cookies) -> Redirect {
        cookies.remove_private(Cookie::named("Authorization"));
        Redirect::to("/login")
    }
}

mod admin {
    use super::*;
    
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
    fn get(user: User) -> Result<Template, Failure> {
        if !user.is_admin {
            return Err(Failure(Status::Forbidden))
        }
        let user_vector: Vec<User> = user::table.get_results(&get_connection())
            .or(Err(Failure(Status::InternalServerError)))?;
        Ok(Template::render("admin.min", &hashmap!["users" => user_vector]))
    }

    #[post("/admin/edituser", data = "<form>")]
    fn edit_user(user: User, form: Form<User>) -> Result<Redirect, Failure> {
        if !user.is_admin {
            return Err(Failure(Status::Forbidden))
        }
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

        diesel::update(user::table.find(model.id))
            .set((user::username.eq(model.username),
                  user::password.eq(model.password),
                  user::is_admin.eq(model.is_admin), ))
            .execute(&get_connection())
            .or(Err(Failure(Status::InternalServerError)))?;
        Ok(Redirect::to("/admin"))
    }
    
    #[post("/admin/deleteuser", data = "<form>")]
    fn delete_user(user: User, form: Form<DeleteForm>) -> Result<Redirect, Failure> {
        if !user.is_admin {
            return Err(Failure(Status::Forbidden))
        }
        let user_id = form.into_inner().id;
        if user.id != 1 && user_id == 1 {
            return Ok(Redirect::to("/thomas/delete"));
        }
        diesel::delete(user::table.find(user_id))
            .execute(&get_connection())
            .or(Err(Failure(Status::InternalServerError)))?;
        Ok(Redirect::to("/admin"))
    }

    #[post("/admin/adduser", data = "<form>")]
    fn add_user(user: User, form: Form<NewData>) -> Result<Redirect, Failure> {
        if !user.is_admin {
            return Err(Failure(Status::Forbidden))
        }
        let mut data = form.into_inner();
        assert_ne!(data.password, "");
        data.password = hash_password(&data.password)
        	.or(Err(Failure(Status::InternalServerError)))?;
        diesel::insert_into(user::table)
            .values(&data)
            .execute(&get_connection())
            .or(Err(Failure(Status::InternalServerError)))?;
        Ok(Redirect::to("/admin"))
    }

    #[derive(Serialize)]
    struct ContextObject {
        id: i32,
        user_id: i32,
        date: NaiveDateTime,
        username: String,
    }

    #[get("/log")]
    fn log(_user: User) -> Result<Template, Failure> {
        if !_user.is_admin {
            return Err(Failure(Status::Forbidden))
        }
        let log_entries: Vec<(LogEntry, User)> = log::table
            .inner_join(user::table)
            .order(log::date.desc())
            .load(&get_connection())
            .or(Err(Failure(Status::InternalServerError)))?;
        let context_objects: Vec<_> = log_entries
            .into_iter()
            .map(|(log_entry, user)| ContextObject {
                id: log_entry.id,
                user_id: log_entry.user_id,
                date: log_entry.date,
                username: user.username,
            }).collect();
        Ok(Template::render("log.min", &hashmap!["log" => context_objects]))
    } 
}

mod thomas {
    use super::*;

    #[get("/thomas/<message>")]
    fn get(message: String) -> Template {
        let message = match message.as_str() {
            "delete" => "mijn account moet verwijderen thomas!",
            "admin" => "mijn adminrechten mag verwijderen thomas!",
            _ => "wat voor shit je ook aan het doen bent doen",
        };
        Template::render("nee_thomas.min", &hashmap!["message" => message])
    }
}
 
fn main()  {
    minify::minify().unwrap();
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![files,
                            index::get, index::post, 
                            login::get, login::post,
                            logout::get, logout::post, 
                            admin::get, admin::edit_user, 
                            admin::add_user, admin::delete_user, 
                            admin::log, 
                            thomas::get, ])
        .catch(errors![catchers::bad_request, catchers::unauthorized, 
                       catchers::forbidden, catchers::not_found, 
                       catchers::unprocessable, catchers::internal, ])
        .launch();
}
