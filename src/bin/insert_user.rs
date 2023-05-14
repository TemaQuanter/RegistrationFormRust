use registration_form_rust::utils::database_functions::{establish_connection, insert_user};

fn main() {
    let connection = &mut establish_connection().unwrap();

    let username = "Mary Roger".to_string();
    let password = "masha123".to_string();

    // let user = insert_user(connection, username, password, None).unwrap();

    println!(
        "Saved a user with\nusername: {}\npassword: {}\n",
        user.username, user.password
    );
}
