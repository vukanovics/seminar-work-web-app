use rocket::{get, http::CookieJar, State};
use rocket_dyn_templates::Template;

use crate::application::{ErrorResponder, BaseLayoutContext, SharedState};

#[get("/")]
pub fn get(
    jar: &CookieJar,
    state: &State<SharedState>,
) -> Result<Template, ErrorResponder> {
    let context = BaseLayoutContext::new(state, jar)?;
    Ok(Template::render("index", context))
}
