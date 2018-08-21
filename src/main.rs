#![feature(plugin, custom_derive, proc_macro_non_items)]
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
use rocket::{response::{self, NamedFile, Response, Responder, }, request::Request, };
use std::{path::{Path, PathBuf, }, };

// wrapper for a named file, but with Cache-Control header
struct CachedFile(NamedFile);

impl<'r> Responder<'r> for CachedFile {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        Response::build_from(self.0.respond_to(request)?)
            //cache for one month
            .raw_header("Cache-Control", "max-age=2628000")
            .ok()
    }
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<CachedFile> {
    NamedFile::open(Path::new("static/")
        .join(file))
        .ok()
        .map(|named_file| CachedFile(named_file))
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
