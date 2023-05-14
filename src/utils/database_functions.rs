use crate::models::User;
use axum::http::StatusCode;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::NewUser;
use crate::schema::users;

/// This function tries to establish a connection with a database.
pub fn establish_connection() -> Result<PgConnection, StatusCode> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    PgConnection::establish(&database_url).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

/// This function tries to insert a new user in a database.
pub fn insert_user(connection: &mut PgConnection, new_user: NewUser) -> Result<User, StatusCode> {
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connection)
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

/// This function tries to remove a user from a database.
pub fn delete_user(connection: &mut PgConnection, user_id: i32) -> Result<(), StatusCode> {
    diesel::delete(users::table)
        .filter(users::columns::id.eq(user_id))
        .execute(connection)
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
        .map(|_res| ())
}
