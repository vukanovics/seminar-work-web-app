#![feature(decl_macro)]

mod application;
mod database;
mod index;
mod register;

use rocket::{ignite, routes};

use application::{ApplicationError, SharedState};

fn main() -> Result<(), ApplicationError> {
    let shared_state = SharedState::new()?;
    ignite()
        .mount("/", routes![index::get, register::get, register::post])
        .manage(shared_state)
        .launch();
    Ok(())
}
