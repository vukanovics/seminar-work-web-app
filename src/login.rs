use rand::{rngs::StdRng, Fill, SeedableRng};
use rocket::{
    form::Form,
    get,
    http::{Cookie, CookieJar},
    post, FromForm, State,
};
use rocket_dyn_templates::Template;
use serde::{self, Serialize};

use crate::application::{BaseLayoutContext, Error, ErrorResponder, SharedState};

#[derive(Serialize, Debug)]
struct LoginLayoutContext {
    #[serde(flatten)]
    base_context: BaseLayoutContext,
    previous_username_or_email: Option<String>,
    error: Option<String>,
    success: Option<String>,
}

impl LoginLayoutContext {
    pub fn new(state: &State<SharedState>, jar: &CookieJar) -> Result<LoginLayoutContext, Error> {
        Ok(LoginLayoutContext {
            base_context: BaseLayoutContext::new(state, jar)?,
            previous_username_or_email: None,
            error: None,
            success: None,
        })
    }

    pub fn with_previous_username_or_email(
        mut self,
        previous_username_or_email: Option<String>,
    ) -> Self {
        self.previous_username_or_email = previous_username_or_email;
        self
    }

    pub fn with_error(mut self, error: Option<String>) -> Self {
        self.error = error;
        self
    }

    pub fn with_success(mut self, success: Option<String>) -> Self {
        self.success = success;
        self
    }
}

#[get("/login")]
pub fn get(state: &State<SharedState>, jar: &CookieJar) -> Result<Template, ErrorResponder> {
    Ok(Template::render(
        "login",
        LoginLayoutContext::new(state, jar)?,
    ))
}

#[derive(FromForm)]
#[allow(clippy::module_name_repetitions)]
pub struct LoginForm {
    pub username_or_email: String,
    pub password: String,
}

impl LoginForm {
    pub fn all_fields_populated(&self) -> bool {
        !self.username_or_email.is_empty() && !self.password.is_empty()
    }
}

fn generate_session_key() -> Result<Vec<u8>, rand::Error> {
    let mut session_key = Vec::new();
    session_key.resize(32, 0u8);
    session_key.try_fill(&mut StdRng::from_entropy())?;
    Ok(session_key)
}

#[allow(clippy::needless_pass_by_value)]
#[post("/login", data = "<data>")]
pub fn post(
    jar: &CookieJar,
    state: &State<SharedState>,
    data: Form<LoginForm>,
) -> Result<Template, ErrorResponder> {
    if let Some(error_message) = 'requirements: {
        if !data.all_fields_populated() {
            break 'requirements Some("All fields are required!");
        }

        let user_by_name = state
            .lock()
            .unwrap()
            .database()
            .get_user_by_username(&data.username_or_email)?;

        let user_by_email = state
            .lock()
            .unwrap()
            .database()
            .get_user_by_email(&data.username_or_email)?;

        let Some(user) = user_by_name.or(user_by_email) else {
            break 'requirements Some("Invalid username/e-mail or password provided.");
        };

        if !bcrypt::verify(&data.password, &user.password).map_err(Error::Bcrypt)? {
            break 'requirements Some("Invalid username/e-mail or password provided.");
        }

        let session_key = generate_session_key().map_err(Error::Rand)?;

        jar.add(Cookie::new("session-key", hex::encode(session_key.clone())));

        state
            .lock()
            .unwrap()
            .database()
            .create_session(crate::models::Session {
                session_key,
                user_id: user.id,
            })?;

        None
    } {
        return Ok(Template::render(
            "login",
            LoginLayoutContext::new(state, jar)?
                .with_previous_username_or_email(Some(data.username_or_email.clone()))
                .with_error(Some(error_message.to_owned())),
        ));
    }

    Ok(Template::render(
        "login",
        LoginLayoutContext::new(state, jar)?
            .with_success(Some("Logged in successfully!".to_string())),
    ))
}
