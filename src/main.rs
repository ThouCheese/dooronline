#![feature(proc_macro_hygiene, decl_macro)]
// #![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod api;
mod crypto;
mod db;
mod macros;
mod models;
mod schema;
mod views;

use rocket::{
    request::Request,
    response::{self, NamedFile, Responder, Response},
    http::Method,
};
use rocket_cors::{self, AllowedHeaders, AllowedOrigins};
use std::path::{Path, PathBuf};
use views::*;

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
    NamedFile::open(Path::new("static/").join(file))
        .ok()
        .map(|named_file| CachedFile(named_file))
}

fn main() {
    rocket::ignite()
        .register(catchers![
            catchers::bad_request,
            catchers::unauthorized,
            catchers::forbidden,
            catchers::not_found,
            catchers::unprocessable,
            catchers::internal,
        ])
        .mount(
            "/",
            routes![
                files,
                index::get,
                index::post,
                login::get,
                login::post,
                logout::get,
                logout::post,
                admin::get,
                admin::edit_user,
                admin::add_user,
                admin::delete_user,
                admin::log,
                thomas::get,
            ],
        )
        .mount("/api/door", routes![api::door::open])
        .mount("/api/log", routes![api::log::list])
        .mount(
            "/api/user",
            routes![
                api::user::login,
                api::user::create,
                api::user::update,
                api::user::list,
                api::user::get
            ],
        )
        .attach(db::DeurDB::fairing())
        .attach(get_cors())
        .launch();
}

fn get_cors() -> rocket_cors::Cors {
    rocket_cors::Cors {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
            "Access-Control-Request-Headers",
            "Access-Control-Request-Method",
            "Origin",
            "User-Agent",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
}
