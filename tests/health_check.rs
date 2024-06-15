// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from  having to specify the `#[test]` attribute.
//
// You can inspect what code gets generate using
// `cargo expand --test health_check` (<- name of the test file)

use std::net::TcpListener;

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


// Launch our application in the backgroud -somehow-
fn spawn_app() -> String{
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind random port");
    // We retrive the port assigned to us bt the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::fun(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller
    format!("http://0.0.0.0:{}", port)
}