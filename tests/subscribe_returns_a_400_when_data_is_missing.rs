//! tests/subscribe_returns_a_400_when_data_is_missing

use nb_backend::spawn_app;
use reqwest;

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    //Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=joe", "missing the email"),
        ("email=dylan%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
        ("name=joe&joe@madhat.org", "bad url formatting"),
    ];

    for (invalid_body, error_message) in test_cases {
        //Act
        let response = client
            .post(&format!("{}/sign_up", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        //Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
