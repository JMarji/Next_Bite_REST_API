//! tests/health_check.rs

use nb_backend::spawn_app;
use reqwest;
use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    //Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", &address))
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
