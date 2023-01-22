use rocket::{get, http::CookieJar, State};
use rocket_dyn_templates::{context, Template};

use crate::application::{ApplicationErrorResponder, SharedState};

#[get("/")]
pub fn get(
    jar: &CookieJar,
    state: &State<SharedState>,
) -> Result<Template, ApplicationErrorResponder> {
    'requirements: {
        let session_key = match jar.get("session-key") {
            Some(session_key) => session_key.value(),
            None => break 'requirements,
        };
        let session_key = match hex::decode(session_key) {
            Ok(session_key) => session_key,
            Err(_) => break 'requirements,
        };
        let session = match state
            .lock()
            .unwrap()
            .database()
            .get_session_by_key(session_key)?
        {
            Some(session) => session,
            None => break 'requirements,
        };
        let user = match state
            .lock()
            .unwrap()
            .database()
            .get_user_by_id(session.user_id)?
        {
            Some(user) => user,
            None => break 'requirements,
        };

        return Ok(Template::render("index", context! {username: user.username}));
    }

    Ok(Template::render("index", context! {}))
}
