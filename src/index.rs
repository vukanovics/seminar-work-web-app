use rocket::get;

#[get("/")]
pub fn get() -> String {
    return "Hello world!".to_string();
}
