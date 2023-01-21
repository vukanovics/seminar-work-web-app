use diesel::result::Error as DieselError;
use rocket::Responder;
use rocket_dyn_templates::{context, Template};

use crate::database::Database;

#[derive(Debug)]
pub enum ApplicationError {
    MissingDatabaseUrl,
    UnableToConnectToDatabase,
    FailedOnADatabaseQuery(DieselError),
    FailedToHashPassword,
}

impl From<DieselError> for ApplicationError {
    fn from(value: DieselError) -> Self {
        Self::FailedOnADatabaseQuery(value)
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
            ApplicationError::FailedToHashPassword => {
                ErrorMessage::Reference("Failed to hash password")
            }
            ApplicationError::FailedOnADatabaseQuery(diesel_error) => {
                ErrorMessage::String(format!("Failed on a database query: {diesel_error}"))
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
