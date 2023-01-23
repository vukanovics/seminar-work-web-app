#![feature(decl_macro)]
#![feature(result_flattening)]

#![warn(clippy::pedantic)]
#![deny(warnings)]
#![allow(clippy::no_effect_underscore_binding)]
mod application;
mod database;
mod index;
mod login;
mod logout;
mod register;

mod models;
mod schema;

use std::sync::Mutex;

use dotenvy::dotenv;
use rocket::{build, fs::FileServer, launch, routes};
use rocket_dyn_templates::Template;
//use rocket_contrib::StaticFiles;

use application::{Error, SharedStateData};

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let shared_state = Mutex::new(SharedStateData::new().unwrap());
    build()
        .mount("/", FileServer::from("static"))
        .mount(
            "/",
            routes![
                index::get,
                register::get,
                register::post,
                login::get,
                login::post,
                logout::get
            ],
        )
        .attach(Template::fairing())
        .manage(shared_state)
}
