// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        content -> Mediumtext,
    }
}

diesel::table! {
    sessions (session_key) {
        session_key -> Binary,
        user_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    sessions,
    users,
);
