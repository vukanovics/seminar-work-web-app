use rocket::{form::Form, get, post, FromForm, State};
use rocket_dyn_templates::{context, Template};

use crate::{
    application::{ApplicationError, ApplicationErrorResponder, SharedState},
    models::NewUser,
};

#[get("/register")]
pub fn get(_state: &State<SharedState>) -> Template {
    Template::render("register", context! {})
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

    pub fn username_is_free(&self, state: &State<SharedState>) -> Result<bool, ApplicationError> {
        Ok(state
            .lock()
            .unwrap()
            .database()
            .get_user_by_username(&self.username)?
            .is_none())
    }

    pub fn email_is_free(&self, state: &State<SharedState>) -> Result<bool, ApplicationError> {
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
    data: Form<RegisterForm>,
) -> Result<Template, ApplicationErrorResponder> {
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
            context! {
                error: error_message,
                previous_username: data.username.clone(),
                previous_email: data.email.clone()
            },
        ));
    }

    let hashed_password = bcrypt::hash(data.password.clone(), bcrypt::DEFAULT_COST)
        .map_err(|_| ApplicationError::FailedToHashPassword)?;

    let new_user = NewUser {
        username: &data.username,
        email: &data.email,
        password: &hashed_password,
    };

    state.lock().unwrap().database().create_user(new_user)?;

    Ok(Template::render(
        "register",
        context! {
            success: "Account successfully registered!",
        },
    ))
}
