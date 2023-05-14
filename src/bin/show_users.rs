use diesel::prelude::*;
use registration_form_rust::models::*;
use registration_form_rust::utils::database_functions::establish_connection;

fn main() {
    use registration_form_rust::schema::users::dsl::*;

    let connection = &mut establish_connection().unwrap();

    let results = users.load::<User>(connection).unwrap();

    for user in results {
        println!(
            "id: {}; username: {}; password: {}",
            user.id, user.username, user.password
        );
    }
}
