pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::User;
use std::env;

use crate::models::NewUser;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|error| panic!("Error connecting to {}", error))
}

pub fn insert_user(
    connection: &mut PgConnection,
    username: String,
    password: String,
    token: Option<String>,
) -> User {
    use crate::schema::users;

    let new_user = NewUser {
        username,
        password,
        token,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error saving a new user")
}
