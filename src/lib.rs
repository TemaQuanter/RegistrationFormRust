pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|error| panic!("Error connecting to {}", error))
}
