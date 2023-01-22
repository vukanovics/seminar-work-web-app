use rand::{rngs::StdRng, Fill, SeedableRng};
use rocket::{
    form::Form,
    get,
    http::{Cookie, CookieJar},
    post, FromForm, State,
};
use rocket_dyn_templates::{context, Template};

use crate::application::{ApplicationError, ApplicationErrorResponder, SharedState};

#[get("/login")]
pub fn get(_state: &State<SharedState>) -> Template {
    Template::render("login", context! {})
}

#[derive(FromForm)]
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

#[post("/login", data = "<data>")]
pub fn post(
    jar: &CookieJar,
    state: &State<SharedState>,
    data: Form<LoginForm>,
) -> Result<Template, ApplicationErrorResponder> {
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

        let user = match user_by_name.or(user_by_email) {
            Some(user) => user,
            None => break 'requirements Some("Invalid username/e-mail or password provided."),
        };

        if !bcrypt::verify(&data.password, &user.password)
            .map_err(ApplicationError::FailedOnABcryptFunction)?
        {
            break 'requirements Some("Invalid username/e-mail or password provided.");
        }

        let session_key = generate_session_key().map_err(ApplicationError::RandError)?;

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
            context! {
                error: error_message,
                previous_username_or_email: data.username_or_email.clone(),
            },
        ));
    }

    Ok(Template::render(
        "login",
        context! {
            success: "Logged in successfully!",
        },
    ))
}
