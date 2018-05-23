#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate sysfs_gpio;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate argon2rs;
extern crate jsonwebtoken as jwt;
extern crate rand;
extern crate serde;
#[macro_use] extern crate serde_derive;

use rocket::{request::Form, 
             response::{Redirect, Failure, NamedFile, }, 
             http::{Cookie, Cookies, Status, }, };
use rocket_contrib::Template;
use models::User;
use schema::user;
use db::get_connection;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

mod models;
mod schema;
mod db;
mod crypto;

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
	if file.to_str().or(None)?.contains("..") {
		panic!("Attempt to access root directory")
	}
    NamedFile::open(Path::new("static/").join(file)).ok()
}

mod catchers {
    use rocket_contrib::Template;  
    use std::collections::HashMap;  
    
    #[error(400)]
    fn bad_request() -> Template {
        let mut context = HashMap::new();
        context.insert("422", false);
        Template::render("400", &context)
    }
    
    #[error(401)]
    fn unauthorized() -> Template {
        Template::render("401", &())
    }
    
    #[error(403)]
    fn forbidden() -> Template {
        Template::render("403", &())
    }
    
    #[error(404)]
    fn not_found() -> Template {
        Template::render("404", &())
    }
    
    #[error(422)]
    fn unprocessable() -> Template {
        let mut context = HashMap::new();
        context.insert("422", true);
        Template::render("400", &context)
    }
    
    #[error(500)]
    fn internal() -> Template {
        Template::render("500", &())
    }
}

mod index {
    use super::*;
    use sysfs_gpio::{Direction, Pin};
    use std::{thread::sleep, time::Duration};

    #[get("/")]
    fn get(_user: User) -> Template {
        Template::render("index", &())
    }

    #[post("/")]
    fn post(_user: User) -> Result<Template, Failure> {
        let my_led = Pin::new(27);
        my_led.with_exported(|| {
            my_led.set_direction(Direction::Out)?;
            my_led.set_value(1)?;
            sleep(Duration::from_millis(1000));
            my_led.set_value(0)?;
            Ok(())
        }).or(Err(Failure(Status::InternalServerError)))?;
        Ok(Template::render("index", &()))
    }
}

mod login {
    use super::*;
    use diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};
    
    #[derive(FromForm)]
    struct LoginData {
        username: String,
        password: String,
    }

    #[get("/login")]
    fn get() -> Template {
        let mut context = HashMap::new();
        context.insert("failed-attempt", false);
        Template::render("login", &context)
    }

    #[post("/login", data="<form>")]
    fn post(form: Form<LoginData>, mut cookies: Cookies) -> Result<Redirect, Template> {
        let user: User = user::table
            .filter(user::username.eq(&form.get().username.to_lowercase()))
            .get_result(&db::get_connection())
            .or_else(|_| {
            		let mut context = HashMap::new();
            		context.insert("message", "Wow wie is dat uberhaupt?");
            		Err(Template::render("login", &context))
            	})?;
        if user.validate_password(&form.get().password) {
            cookies.add_private(Cookie::new("Authorization", user.create_jwt()
            	.or_else(|_| {
            		let mut context = HashMap::new();
            		context.insert("message", "Geen logintoken voor jou haha");
            		Err(Template::render("login", &context))	
            	})?));
            Ok(Redirect::to("/"))
        } else {
            let mut context = HashMap::new();
            context.insert("message", "Je wachtwoord is kut en fout");
            Err(Template::render("login", &context))
        }
    }
}

mod logout {
    use super::*;
    
    #[get("/logout")]
    fn get() -> Template {
        Template::render("logout", &()) 
    }

    #[post("/logout")]
    fn post(_user: User, mut cookies: Cookies) -> Redirect {
        cookies.remove_private(Cookie::named("Authorization"));
        Redirect::to("/login")
    }
}

mod admin {
    use super::*;
    use diesel::{RunQueryDsl, QueryDsl};
    use crypto::hash_password;
    use schema::user;
    
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
        if user.is_admin {
            let user_vector: Vec<User> = user::table.get_results(&get_connection())
            	.or(Err(Failure(Status::InternalServerError)))?;
            let mut context = HashMap::new();
            context.insert("users", user_vector);
            Ok(Template::render("admin", &context))
        } else {
            Err(Failure(rocket::http::Status::Forbidden))
        }
    }

    #[post("/admin/edituser", data = "<form>")]
    fn edit_user(user: User, form: Form<User>) -> Result<Redirect, Failure> {
        use diesel::ExpressionMethods;

        if user.is_admin {
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
        } else {
            Err(Failure(Status::Forbidden))
        }
    }
    
    #[post("/admin/deleteuser", data = "<form>")]
    fn delete_user(user: User, form: Form<DeleteForm>) -> Result<Redirect, Failure> {
        if user.is_admin {
            let user_id = form.into_inner().id;
            if user.id != 1 && user_id == 1 {
                return Ok(Redirect::to("/thomas/delete"));
            }
            diesel::delete(user::table.find(user_id))
                .execute(&get_connection())
                .or(Err(Failure(Status::InternalServerError)))?;
            Ok(Redirect::to("/admin"))
        } else {
            Err(Failure(Status::Forbidden))
        }
    }

    #[post("/admin/adduser", data = "<form>")]
    fn add_user(user: User, form: Form<NewData>) -> Result<Redirect, Failure> {
        if user.is_admin {
            let mut data = form.into_inner();
            assert_ne!(data.password, "");
            data.password = hash_password(&data.password)
            	.or(Err(Failure(Status::InternalServerError)))?;
            diesel::insert_into(user::table)
                .values(&data)
                .execute(&get_connection())
                .or(Err(Failure(Status::InternalServerError)))?;
            Ok(Redirect::to("/admin"))
        } else {
            Err(Failure(Status::Forbidden))
        }
    }
}

mod thomas {
    use rocket_contrib::Template;
    use std::collections::HashMap;

    #[get("/thomas/<message>")]
    fn get(message: String) -> Template {
        let mut context = HashMap::new();
        context.insert("message", match message.as_str() {
            "delete" => "mijn account moet verwijderen thomas!",
            "admin" => "mijn adminrechten mag verwijderen thomas!",
            _ => "wat voor shit je ook aan het doen bent doen",
        });
        Template::render("nee_thomas", &context)
    }
}
 
fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![files,
                            index::get, index::post, 
                            login::get, login::post,
                            logout::get, logout::post, 
                            admin::get, admin::edit_user, 
                            admin::add_user, admin::delete_user,
                            thomas::get, ])
        .catch(errors![catchers::bad_request, catchers::unauthorized, 
                       catchers::forbidden, catchers::not_found, 
                       catchers::unprocessable, catchers::internal, ])
        .launch();
}

