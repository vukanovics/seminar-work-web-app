#![feature(decl_macro)]

mod application;
mod database;
mod index;
mod register;

mod models;
mod schema;

use std::sync::Mutex;

use dotenvy::dotenv;
use rocket::{build, launch, routes};
use rocket_dyn_templates::Template;

use application::{ApplicationError, SharedStateData};

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let shared_state = Mutex::new(SharedStateData::new().unwrap());
    build()
        .mount("/", routes![index::get, register::get, register::post])
        .attach(Template::fairing())
        .manage(shared_state)
}
