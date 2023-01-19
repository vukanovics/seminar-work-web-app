#![feature(decl_macro)]

mod index;

use rocket::{ignite, routes};

fn main() {
    ignite().mount("/", routes![index::get]).launch();
}
