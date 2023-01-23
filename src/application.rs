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
    FailedOnADatabaseQuery(DieselError),
    FailedOnABcryptFunction(BcryptError),
    RandError(rand::Error),
}

impl From<DieselError> for Error {
    fn from(value: DieselError) -> Self {
        Self::FailedOnADatabaseQuery(value)
    }
}

impl From<BcryptError> for Error {
    fn from(value: BcryptError) -> Self {
        Self::FailedOnABcryptFunction(value)
    }
}

#[derive(Responder)]
pub struct ApplicationErrorResponder {
    result: Template,
}

impl From<Error> for ApplicationErrorResponder {
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
            Error::FailedOnABcryptFunction(bcrypt_error) => {
                ErrorMessage::String(format!("Failed on a bcrypt function: {bcrypt_error}"))
            }
            Error::FailedOnADatabaseQuery(diesel_error) => {
                ErrorMessage::String(format!("Failed on a database query: {diesel_error}"))
            }
            Error::RandError(rand_error) => {
                ErrorMessage::String(format!("Rand error: {rand_error}"))
            }
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
}

#[derive(Serialize, Debug)]
pub struct BaseLayoutContext {
    username: Option<String>,
}

impl BaseLayoutContext {
    fn get_valid_user_info_from_session_cookie(
        state: &State<SharedState>,
        cookie: &Cookie<'static>,
    ) -> Result<Option<User>, Error> {
        hex::decode(cookie.value())
            .ok()
            // attempt to get a session with that session key
            .and_then(|decoded_key| {
                state
                    .lock()
                    .unwrap()
                    .database()
                    .get_session_by_key(decoded_key)
                    .transpose()
            })
            // if the session exists, attempt to get a user with the session's user_id
            .and_then(|maybe_session| {
                maybe_session
                    .map(|session| {
                        state
                            .lock()
                            .unwrap()
                            .database()
                            .get_user_by_id(session.user_id)
                            .transpose()
                    })
                    .transpose()
            })
            // merge the two Results into one
            .map(|user| user.flatten())
            .transpose()
    }
    fn get_valid_user_info(
        state: &State<SharedState>,
        jar: &CookieJar,
    ) -> Result<Option<User>, Error> {
        // attempt the current session key cookie
        jar.get("session-key")
            .map(|cookie| Self::get_valid_user_info_from_session_cookie(state, cookie))
            // or if that fails, try the one not yet sent to the user
            .or_else(|| {
                jar.get_pending("session-key")
                    .map(|cookie| Self::get_valid_user_info_from_session_cookie(state, &cookie))
            })
            .transpose()
            // combine the two options into one
            .map(|either_session_key| either_session_key.flatten())
    }

    pub fn new(
        state: &State<SharedState>,
        jar: &CookieJar,
    ) -> Result<BaseLayoutContext, Error> {
        let user_info = Self::get_valid_user_info(state, jar)?;
        Ok(BaseLayoutContext {
            username: user_info.map(|info| info.username),
        })
    }
}
