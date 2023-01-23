use chrono::NaiveDateTime;

use diesel::prelude::*;
use serde::Serialize;

use crate::schema::{posts, sessions, users};

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

#[derive(Queryable, Debug, Serialize)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub author: i32,
    pub created_on: NaiveDateTime,
    pub title: String,
    pub description: String,
    pub content: String,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub author: i32,
    pub created_on: NaiveDateTime,
    pub description: &'a str,
    pub content: &'a str,
}
