use registration_form_rust::run;

#[tokio::main]
async fn main() {
    // Run the main function which makes the server up and
    // continues until termination.
    run().await;
}
