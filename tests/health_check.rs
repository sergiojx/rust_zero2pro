// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from  having to specify the `#[test]` attribute.
//
// You can inspect what code gets generate using
// `cargo expand --test health_check` (<- name of the test file)

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    // We need to bring in `equest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://0.0.0.0:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


// Launch our application in the backgroud -somehow-
fn spawn_app() {
    let server = zero2prod::fun().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}