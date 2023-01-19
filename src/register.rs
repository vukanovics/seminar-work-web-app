use rocket::{get, post, State};

use crate::application::SharedState;

#[get("/register")]
pub fn get(_state: State<SharedState>) -> String {
    return "Hello world!".to_string();
}

#[post("/register")]
pub fn post(_state: State<SharedState>) -> String {
    return "Hello world!".to_string();
}
