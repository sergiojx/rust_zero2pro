// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from  having to specify the `#[test]` attribute.
//
// You can inspect what code gets generate using
// `cargo expand --test health_check` (<- name of the test file)

use std::{clone, net::TcpListener};

use reqwest::Client;

use std::env;
use sqlx::{PgConnection, Connection};
use zero2prod::configuration::get_configuration;
// Launch our application in the backgroud -somehow-
// Spin up an instance of our application
// and returns its address (i.e http://localhost:XXXX)
fn spawn_app() -> String{
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind random port");
    // We retrive the port assigned to us bt the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::fun(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller
    format!("http://0.0.0.0:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    // We need to bring in `equest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


#[tokio::test]
async fn subcribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();

    // data base connection
    let configuration = get_configuration().expect("Failed to read configuration");
    let db_connection_string = configuration.database.configuration_string();
    // The `Connection` trait MUST be in scope for us to invoke
    // `PgConnection::connect` - it is not an inherent method of the struct!
    // println!("db_connection_string: {}", db_connection_string);
    let mut db_connection = PgConnection::connect(&db_connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let client = reqwest::Client::new();
    
    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
            .post(&format!("{}/subscriptions",&app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut db_connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le%20guin");

}

#[tokio::test]
async fn subcribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for(invalid_body, error_message) in test_cases {
        // Act
        let response = client
                .post(&format!("{}/subscriptions",&app_address))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(invalid_body)
                .send()
                .await
                .expect("Failed to execute request.");


        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}
