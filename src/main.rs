#![feature(plugin, custom_derive, proc_macro_non_items, use_extern_macros)]
#![plugin(rocket_codegen)]
#![allow(proc_macro_derive_resolution_fallback)]

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
extern crate chrono;
extern crate maud;

mod models;
mod schema;
mod db;
mod crypto;
mod minify;
#[macro_use]
mod macros;
mod views;

use views::*;
use rocket::response::NamedFile;
use std::{path::{Path, PathBuf, }, };

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    if file.to_str().or(None)?.contains("..") {
        panic!("Attempt to access root directory")
    }
    NamedFile::open(Path::new("static/").join(file)).ok()
}
 
fn main()  {
    minify::minify().unwrap();
    rocket::ignite()
        .mount("/", routes![files,
                            index::get, index::post, 
                            login::get, login::post,
                            logout::get, logout::post, 
                            admin::get, admin::edit_user, 
                            admin::add_user, admin::delete_user, 
                            admin::log, 
                            thomas::get, ])
        .catch(catchers![catchers::bad_request, catchers::unauthorized, 
                         catchers::forbidden, catchers::not_found, 
                         catchers::unprocessable, catchers::internal, ])
        .launch();
}
