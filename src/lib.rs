//! lib.rs
// PRE-PROCCESSOR DIRECTIVES
pub mod configuration;
pub mod routes;
pub mod startup;

use std::net::TcpListener;

// END OF PRE-PROCCESSOR DIRECTIVES

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port();

    let server = startup::startup(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
