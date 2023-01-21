use rocket::get;

#[get("/")]
pub fn get() -> String {
    "Hello world!".to_string()
}
