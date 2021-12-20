//! src/startup.rs

use actix_web::dev::Server;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::env;

use crate::routes::health_check;
use crate::routes::sign_up;


pub fn startup(listener: std::net::TcpListener) -> Result<Server, std::io::Error> {
    env::set_var("RUST_LOG", "actix_web=debug");
    //env::set_var("RUST_BACKTRACE", "1");

    color_eyre::install().expect("Failed to install color_eyere");

    //let serv_config = Config::get_config().expect("Server configuration");

    // info!("Starting server at {}", serv_config.host);

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/sign_up", web::post().to(sign_up))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
