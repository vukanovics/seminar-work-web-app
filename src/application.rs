use bcrypt::BcryptError;
use diesel::result::Error as DieselError;
use rocket::{
    http::{Cookie, CookieJar},
    Responder, State,
};
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

use crate::{database::Database, models::User};

#[derive(Debug)]
pub enum Error {
    MissingDatabaseUrl,
    UnableToConnectToDatabase,
    Diesel(DieselError),
    Bcrypt(BcryptError),
    Rand(rand::Error),
    PostHasInvalidUserId,
}

impl From<DieselError> for Error {
    fn from(value: DieselError) -> Self {
        Self::Diesel(value)
    }
}

impl From<BcryptError> for Error {
    fn from(value: BcryptError) -> Self {
        Self::Bcrypt(value)
    }
}

#[derive(Responder)]
pub struct ErrorResponder {
    result: Template,
}

impl From<Error> for ErrorResponder {
    fn from(value: Error) -> Self {
        enum ErrorMessage<'a> {
            Reference(&'a str),
            String(String),
        }
        let message: ErrorMessage = match value {
            Error::MissingDatabaseUrl => {
                ErrorMessage::Reference("Missing database URL in the server configuration")
            }
            Error::UnableToConnectToDatabase => {
                ErrorMessage::Reference("Unable to connect to the provided database URL")
            }
            Error::PostHasInvalidUserId => {
                ErrorMessage::Reference("Post has an invalid user id")
            }
            Error::Bcrypt(bcrypt_error) => {
                ErrorMessage::String(format!("Failed on a bcrypt function: {bcrypt_error}"))
            }
            Error::Diesel(diesel_error) => {
                ErrorMessage::String(format!("Failed on a database query: {diesel_error}"))
            }
            Error::Rand(rand_error) => ErrorMessage::String(format!("Rand error: {rand_error}")),
        };

        let result = match message {
            ErrorMessage::Reference(message) => {
                Template::render("server_error", context! {error_message: message})
            }
            ErrorMessage::String(message) => {
                Template::render("server_error", context! {error_message: message})
            }
        };

        Self { result }
    }
}

pub struct SharedStateData {
    database: Database,
}

use std::sync::Mutex;
pub type SharedState = Mutex<SharedStateData>;

impl SharedStateData {
    pub fn new() -> Result<SharedStateData, Error> {
        Ok(SharedStateData {
            database: Database::new()?,
        })
    }

    pub fn database(&mut self) -> &mut Database {
        &mut self.database
    }

    fn get_valid_user_info_from_session_cookie(
        &mut self,
        cookie: &Cookie<'static>,
    ) -> Result<Option<User>, Error> {
        hex::decode(cookie.value())
            .ok()
            // attempt to get a session with that session key
            .and_then(|decoded_key| self.database().get_session_by_key(decoded_key).transpose())
            // if the session exists, attempt to get a user with the session's user_id
            .and_then(|maybe_session| {
                maybe_session
                    .map(|session| self.database().get_user_by_id(session.user_id).transpose())
                    .transpose()
            })
            // merge the two Results into one
            .map(Result::flatten)
            .transpose()
    }

    pub fn get_valid_user_info(&mut self, jar: &CookieJar) -> Result<Option<User>, Error> {
        // get a not-yet-sent session key if there is one
        let pending = jar
            .get_pending("session-key")
            .map(|cookie| self.get_valid_user_info_from_session_cookie(&cookie))
            .transpose()?
            .flatten();
        Ok(jar
            .get("session-key")
            .map(|cookie| self.get_valid_user_info_from_session_cookie(cookie))
            .transpose()?
            .flatten()
            // try the pending one if the received one is invalid/doesn't exist
            .or_else(|| pending))
    }
}

#[derive(Serialize, Debug)]
pub struct BaseLayoutContext {
    username: Option<String>,
}

impl BaseLayoutContext {
    pub fn new(state: &State<SharedState>, jar: &CookieJar) -> Result<BaseLayoutContext, Error> {
        let user_info = state.lock().unwrap().get_valid_user_info(jar)?;
        Ok(BaseLayoutContext {
            username: user_info.map(|info| info.username),
        })
    }
}
