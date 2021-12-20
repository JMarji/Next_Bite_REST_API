//! tests/health_check.rs

use nb_backend::startup::spawn_app;
use reqwest;

#[actix_rt::test]
async fn health_check_works() {
    //Arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

/***************************************************************************
Spin up an instance of our application and returns it's address
(i.e. http://localhost:XXXX)
**************************************************************************** */
