use std::env;

use diesel::{mysql::MysqlConnection, prelude::*, Connection, RunQueryDsl};

use crate::{
    models::{NewUser, Session, User},
    schema::{sessions, users},
    ApplicationError,
};

pub struct Database {
    connection: MysqlConnection,
}

impl Database {
    pub fn new() -> Result<Database, ApplicationError> {
        let database_url =
            env::var("DATABASE_URL").map_err(|_| ApplicationError::MissingDatabaseUrl)?;

        let connection = MysqlConnection::establish(&database_url)
            .map_err(|_| ApplicationError::UnableToConnectToDatabase)?;

        Ok(Database { connection })
    }

    fn diesel_result_to_option<T>(
        value: Result<T, diesel::result::Error>,
    ) -> Result<Option<T>, ApplicationError> {
        match value {
            Ok(value) => Ok(Some(value)),
            Err(error) => match error {
                diesel::result::Error::NotFound => Ok(None),
                _ => Err(ApplicationError::FailedOnADatabaseQuery(error)),
            },
        }
    }

    pub fn get_user_by_id(&mut self, by_id: i32) -> Result<Option<User>, ApplicationError> {
        use crate::schema::users::dsl::*;
        Self::diesel_result_to_option(
            users
                .filter(id.eq(by_id))
                .limit(1)
                .first::<User>(&mut self.connection),
        )
    }

    pub fn get_user_by_username(
        &mut self,
        by_username: &str,
    ) -> Result<Option<User>, ApplicationError> {
        use crate::schema::users::dsl::*;
        Self::diesel_result_to_option(
            users
                .filter(username.eq(by_username))
                .limit(1)
                .first::<User>(&mut self.connection),
        )
    }

    pub fn get_user_by_email(&mut self, by_email: &str) -> Result<Option<User>, ApplicationError> {
        use crate::schema::users::dsl::*;
        Self::diesel_result_to_option(
            users
                .filter(email.eq(by_email))
                .limit(1)
                .first::<User>(&mut self.connection),
        )
    }

    pub fn get_session_by_key(
        &mut self,
        by_key: Vec<u8>,
    ) -> Result<Option<Session>, ApplicationError> {
        use crate::schema::sessions::dsl::*;
        Self::diesel_result_to_option(
            sessions
                .filter(session_key.eq(by_key))
                .limit(1)
                .first::<Session>(&mut self.connection),
        )
    }

    pub fn create_user(&mut self, user: NewUser) -> Result<(), ApplicationError> {
        diesel::insert_into(users::table)
            .values(user)
            .execute(&mut self.connection)?;
        Ok(())
    }

    pub fn create_session(&mut self, session: Session) -> Result<(), ApplicationError> {
        diesel::insert_into(sessions::table)
            .values(session)
            .execute(&mut self.connection)?;
        Ok(())
    }
}
