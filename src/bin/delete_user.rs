use diesel::{ExpressionMethods, RunQueryDsl};
use registration_form_rust::{establish_connection, schema::users};

fn main() {
    let connection = &mut establish_connection();

    let delete_username = "Mary Roger";

    diesel::delete(users::table)
        .filter(users::columns::username.eq(delete_username))
        .execute(connection)
        .expect("Failed to remove an object");

    println!("The user was removed successfully");
}
