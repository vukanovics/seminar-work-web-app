use diesel::prelude::*;

use crate::schema::{sessions, users};

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub session_key: Vec<u8>,
    pub user_id: i32,
}
