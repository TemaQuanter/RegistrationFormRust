use registration_form_rust::{establish_connection, insert_user};

fn main() {
    let connection = &mut establish_connection();

    let username = "Mary Roger".to_string();
    let password = "masha123".to_string();

    let user = insert_user(connection, username, password, None);

    println!(
        "Saved a user with\nusername: {}\npassword: {}\n",
        user.username, user.password
    );
}
