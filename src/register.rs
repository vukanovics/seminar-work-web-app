use rocket::{form::Form, get, http::CookieJar, post, FromForm, State};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{
    application::{BaseLayoutContext, Error, ErrorResponder, SharedState},
    models::NewUser,
};

#[derive(Serialize, Debug)]
struct RegisterLayoutContext {
    #[serde(flatten)]
    base_context: BaseLayoutContext,
    previous_username: Option<String>,
    previous_email: Option<String>,
    error: Option<String>,
    success: Option<String>,
}

impl RegisterLayoutContext {
    pub fn new(
        state: &State<SharedState>,
        jar: &CookieJar,
    ) -> Result<RegisterLayoutContext, Error> {
        Ok(RegisterLayoutContext {
            base_context: BaseLayoutContext::new(state, jar)?,
            previous_username: None,
            previous_email: None,
            error: None,
            success: None,
        })
    }

    pub fn with_previous_username(mut self, previous_username: Option<String>) -> Self {
        self.previous_username = previous_username;
        self
    }

    pub fn with_previous_email(mut self, previous_email: Option<String>) -> Self {
        self.previous_email = previous_email;
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

#[get("/register")]
pub fn get(state: &State<SharedState>, jar: &CookieJar) -> Result<Template, ErrorResponder> {
    Ok(Template::render(
        "register",
        RegisterLayoutContext::new(state, jar)?,
    ))
}

#[derive(FromForm)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_repeat: String,
}

impl RegisterForm {
    pub fn all_fields_populated(&self) -> bool {
        !self.username.is_empty()
            && !self.email.is_empty()
            && !self.password.is_empty()
            && !self.password_repeat.is_empty()
    }

    pub fn both_passwords_same(&self) -> bool {
        self.password == self.password_repeat
    }

    pub fn username_is_free(&self, state: &State<SharedState>) -> Result<bool, Error> {
        Ok(state
            .lock()
            .unwrap()
            .database()
            .get_user_by_username(&self.username)?
            .is_none())
    }

    pub fn email_is_free(&self, state: &State<SharedState>) -> Result<bool, Error> {
        Ok(state
            .lock()
            .unwrap()
            .database()
            .get_user_by_email(&self.email)?
            .is_none())
    }
}

#[post("/register", data = "<data>")]
pub fn post(
    state: &State<SharedState>,
    jar: &CookieJar,
    data: Form<RegisterForm>,
) -> Result<Template, ErrorResponder> {
    let context = RegisterLayoutContext::new(state, jar)?;
    if let Some(error_message) = 'requirements: {
        if !data.all_fields_populated() {
            break 'requirements Some("All fields are required!");
        }
        if !data.both_passwords_same() {
            break 'requirements Some("Password attempts don't match!");
        }
        if !data.username_is_free(state)? {
            break 'requirements Some("Username already in use!");
        }
        if !data.email_is_free(state)? {
            break 'requirements Some("E-mail already in use!");
        }
        None
    } {
        return Ok(Template::render(
            "register",
            context
                .with_error(Some(error_message.to_string()))
                .with_previous_username(Some(data.username.clone()))
                .with_previous_email(Some(data.email.clone())),
        ));
    }

    let hashed_password =
        bcrypt::hash(data.password.clone(), bcrypt::DEFAULT_COST).map_err(Error::Bcrypt)?;

    let new_user = NewUser {
        username: &data.username,
        email: &data.email,
        password: &hashed_password,
    };

    state.lock().unwrap().database().create_user(new_user)?;

    Ok(Template::render(
        "register",
        context.with_success(Some("Account successfully registered!".to_string())),
    ))
}
