use bcrypt::BcryptError;
use diesel::result::Error as DieselError;
use rocket::Responder;
use rocket_dyn_templates::{context, Template};

use crate::database::Database;

#[derive(Debug)]
pub enum ApplicationError {
    MissingDatabaseUrl,
    UnableToConnectToDatabase,
    FailedOnADatabaseQuery(DieselError),
    FailedOnABcryptFunction(BcryptError),
    RandError(rand::Error),
}

impl From<DieselError> for ApplicationError {
    fn from(value: DieselError) -> Self {
        Self::FailedOnADatabaseQuery(value)
    }
}

impl From<BcryptError> for ApplicationError {
    fn from(value: BcryptError) -> Self {
        Self::FailedOnABcryptFunction(value)
    }
}

#[derive(Responder)]
pub struct ApplicationErrorResponder {
    result: Template,
}

impl From<ApplicationError> for ApplicationErrorResponder {
    fn from(value: ApplicationError) -> Self {
        enum ErrorMessage<'a> {
            Reference(&'a str),
            String(String),
        }
        let message: ErrorMessage = match value {
            ApplicationError::MissingDatabaseUrl => {
                ErrorMessage::Reference("Missing database URL in the server configuration")
            }
            ApplicationError::UnableToConnectToDatabase => {
                ErrorMessage::Reference("Unable to connect to the provided database URL")
            }
            ApplicationError::FailedOnABcryptFunction(bcrypt_error) => {
                ErrorMessage::String(format!("Failed on a bcrypt function: {bcrypt_error}"))
            }
            ApplicationError::FailedOnADatabaseQuery(diesel_error) => {
                ErrorMessage::String(format!("Failed on a database query: {diesel_error}"))
            }
            ApplicationError::RandError(rand_error) => {
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
    pub fn new() -> Result<SharedStateData, ApplicationError> {
        Ok(SharedStateData {
            database: Database::new()?,
        })
    }

    pub fn database(&mut self) -> &mut Database {
        &mut self.database
    }
}
