//! src/main.rs

use actix_web;
use nb_backend::configuration::get_configuration;
use nb_backend::startup::startup;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    // We have removed the hard-coded application port - it's now coming our settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    startup(listener)?.await
}
