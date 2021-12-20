//!tests/subscribe_returns_a_200_for_valid_form_data.rs

use nb_backend::configuration::get_configuration;
use nb_backend::spawn_app;
use reqwest;
use sqlx::{Connection, PgConnection};

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //Arange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=joe&email=joemarji%40hotmail.com";
    let configuration = get_configuration().expect("Failed to read configuration");
    //Act
    let response = client
        .post(&format!("{}/sign_up", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    let connection_string = configuration.database.connection_string();
    // The 'Connection' trait Must be in scope for us to invoke
    // 'PgConnection::connect' - it is not an inhearent method of the struct!
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    //Assert

    let saved = sqlx::query!("SELECT email, name FROM users",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved User");

    assert_eq!(200, response.status().as_u16());
    assert_eq!(saved.name, "joe");
}
