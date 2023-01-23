use rocket::{
    get,
    http::{Cookie, CookieJar},
    response::Redirect,
    State,
};

use crate::application::{ErrorResponder, SharedState};

#[get("/logout")]
pub fn get(
    state: &State<SharedState>,
    jar: &CookieJar,
) -> Result<Redirect, ErrorResponder> {
    jar.get("session-key")
        .and_then(|encoded_key| {
            hex::decode(encoded_key.value())
                .map(|session_key| {
                    state
                        .lock()
                        .unwrap()
                        .database()
                        .remove_session_by_key(session_key)
                })
                .ok()
        })
        .transpose()?;

    jar.remove(Cookie::named("session-key"));

    Ok(Redirect::to("/"))
}
