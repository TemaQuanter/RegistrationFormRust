use registration_form_rust::{run, utils::database_functions::establish_connection};

#[tokio::main]
async fn main() {
    // Try to establish a connection with a database, if fails,
    // then panic and exit the program.
    let mut database_connection = if let Ok(connection) = establish_connection() {
        connection
    } else {
        panic!("Failed to establish a connection with a database");
    };

    // Run the main function which makes the server up and
    // continues until termination.
    run(&mut database_connection).await;
}
