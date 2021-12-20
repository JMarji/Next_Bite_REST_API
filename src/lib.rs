//! lib.rs
// PRE-PROCCESSOR DIRECTIVES
pub mod configuration;
pub mod routes;
pub mod startup;

//use crate::vars::Config;
use actix_web::dev::Server;
use actix_web::{
    /*get*/
    middleware::Logger, /*post*/
    web, App, /*HttpRequest*/
    HttpResponse, HttpServer,
    /*Responder*/
};
use color_eyre::Result;
//use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde;
use std::env;
use std::net::TcpListener;
//use tracing::info;
// END OF PRE-PROCCESSOR DIRECTIVES

// ADTs
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}
// HANDLERS
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
// END OF HANDLERS
//noinspection ALL
pub fn run(listener: std::net::TcpListener) -> Result<Server, std::io::Error> {
    env::set_var("RUST_LOG", "actix_web=debug");
    //env::set_var("RUST_BACKTRACE", "1");

    color_eyre::install().expect("Failed to install color_eyere");

    //let serv_config = Config::get_config().expect("Server configuration");

    // info!("Starting server at {}", serv_config.host);

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
