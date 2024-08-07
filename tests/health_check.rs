// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from  having to specify the `#[test]` attribute.
//
// You can inspect what code gets generate using
// `cargo expand --test health_check` (<- name of the test file)

use std::{clone, net::TcpListener};

use reqwest::Client;

use std::env;
use sqlx::{PgConnection, Connection, PgPool};
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use uuid::Uuid;


pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}
// Launch our application in the backgroud -somehow-
// Spin up an instance of our application
// and returns its address (i.e http://localhost:XXXX)
async fn spawn_app() -> TestApp{
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind random port");
    // We retrive the port assigned to us bt the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://0.0.0.0:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let db_connection_pool = configure_database(&configuration.database).await;

    let server = zero2prod::startup::fun(listener, db_connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: db_connection_pool,

    }
    
}


pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(
        &config.configuration_string_without_db()
    )
    .await
    .expect("Failed to create database.");
    
    sqlx::query(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .execute(&mut connection)
        .await
        .expect("Error creating database");
    // connection
    //     .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
    //     .await
    //     .expect("Failed to create database");

    // Migrate database
    let connection_pool = PgPool::connect(&config.configuration_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("failed to migrate the database");

    connection_pool

}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    // We need to bring in `equest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
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
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    
    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
            .post(&format!("{}/subscriptions",&app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");

}

#[tokio::test]
async fn subcribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for(invalid_body, error_message) in test_cases {
        // Act
        let response = client
                .post(&format!("{}/subscriptions",&app.address))
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
