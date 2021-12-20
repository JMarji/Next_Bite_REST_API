//! src/startup.rs

use actix_web::dev::Server;
use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::env;
use std::net::TcpListener;
use uuid::Uuid;

use crate::configuration::{get_configuration, DatabaseSettings};
use crate::routes::{health_check, sign_up};

pub fn run(listener: std::net::TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    env::set_var("RUST_LOG", "actix_web=debug");
    //env::set_var("RUST_BACKTRACE", "1");

    color_eyre::install().expect("Failed to install color_eyere");

    //let serv_config = Config::get_config().expect("Server configuration");

    // info!("Starting server at {}", serv_config.host);
    // Wrap the pool in a smart pointer
    let db_pool = web::Data::new(db_pool);
    // Capture 'connection' from the surrounding environment (move)
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/sign_up", web::post().to(sign_up))
            // Get a pointer copy and attach it to the application state
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

// Test Infrastructure starts here

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configs");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create Database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");
    // Migrate Database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
