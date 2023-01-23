use std::env;

use diesel::{mysql::MysqlConnection, prelude::*, Connection, RunQueryDsl};

use crate::{
    models::{NewPost, NewUser, Post, Session, User},
    schema::{posts, sessions, users},
    Error,
};

pub struct Database {
    connection: MysqlConnection,
}

impl Database {
    pub fn new() -> Result<Database, Error> {
        let database_url = env::var("DATABASE_URL").map_err(|_| Error::MissingDatabaseUrl)?;

        let connection = MysqlConnection::establish(&database_url)
            .map_err(|_| Error::UnableToConnectToDatabase)?;

        Ok(Database { connection })
    }

    fn diesel_result_to_option<T>(
        value: Result<T, diesel::result::Error>,
    ) -> Result<Option<T>, Error> {
        match value {
            Ok(value) => Ok(Some(value)),
            Err(error) => match error {
                diesel::result::Error::NotFound => Ok(None),
                _ => Err(Error::Diesel(error)),
            },
        }
    }

    pub fn get_user_by_id(&mut self, by_id: i32) -> Result<Option<User>, Error> {
        use crate::schema::users::dsl::{id, users};
        Self::diesel_result_to_option(
            users
                .filter(id.eq(by_id))
                .limit(1)
                .first::<User>(&mut self.connection),
        )
    }

    pub fn get_user_by_username(&mut self, by_username: &str) -> Result<Option<User>, Error> {
        use crate::schema::users::dsl::{username, users};
        Self::diesel_result_to_option(
            users
                .filter(username.eq(by_username))
                .limit(1)
                .first::<User>(&mut self.connection),
        )
    }

    pub fn get_user_by_email(&mut self, by_email: &str) -> Result<Option<User>, Error> {
        use crate::schema::users::dsl::{email, users};
        Self::diesel_result_to_option(
            users
                .filter(email.eq(by_email))
                .limit(1)
                .first::<User>(&mut self.connection),
        )
    }

    pub fn get_session_by_key(&mut self, by_key: Vec<u8>) -> Result<Option<Session>, Error> {
        use crate::schema::sessions::dsl::{session_key, sessions};
        Self::diesel_result_to_option(
            sessions
                .filter(session_key.eq(by_key))
                .limit(1)
                .first::<Session>(&mut self.connection),
        )
    }

    pub fn get_latest_x_posts(&mut self, count: i64) -> Result<Vec<Post>, Error> {
        use crate::schema::posts::dsl::{created_on, posts};
        posts
            .limit(count)
            .order(created_on.desc())
            .load::<Post>(&mut self.connection)
            .map_err(Error::Diesel)
    }

    pub fn create_user(&mut self, user: NewUser) -> Result<(), Error> {
        diesel::insert_into(users::table)
            .values(user)
            .execute(&mut self.connection)?;
        Ok(())
    }

    pub fn create_session(&mut self, session: Session) -> Result<(), Error> {
        diesel::insert_into(sessions::table)
            .values(session)
            .execute(&mut self.connection)?;
        Ok(())
    }

    pub fn create_post(&mut self, post: NewPost) -> Result<(), Error> {
        diesel::insert_into(posts::table)
            .values(post)
            .execute(&mut self.connection)?;
        Ok(())
    }

    pub fn remove_session_by_key(&mut self, by_key: Vec<u8>) -> Result<(), Error> {
        use crate::schema::sessions::dsl::{session_key, sessions};
        diesel::delete(sessions)
            .filter(session_key.eq(by_key))
            .execute(&mut self.connection)?;
        Ok(())
    }
}
