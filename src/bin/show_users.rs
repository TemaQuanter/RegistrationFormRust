use diesel::prelude::*;
use registration_form_rust::{establish_connection, models::*};

fn main() {
    use registration_form_rust::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let results = users.load::<User>(connection).unwrap();

    for user in results {
        println!(
            "id: {}; username: {}; password: {}",
            user.id, user.username, user.password
        );
    }
}
