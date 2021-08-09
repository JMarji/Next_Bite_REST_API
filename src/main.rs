#![allow(dead_code, unused_imports)]

use actix_web::{get, post, web, App, HttpResponse, HttpServer, middleware::Logger, Responder};
use color_eyre::Result;
use std::env;
use tracing::info;
use crate::vars::Config;
use crate::register::Register;
use r2d2::Pool;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager, VerifyPeer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod vars;
mod register; 
mod handlers;


#[actix_web::main]
async fn main() -> Result<()> {


    
    env::set_var("RUST_LOG", "actix_web=debug");

    color_eyre::install()?; 

   let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host("localhost", 27020)
            .with_db("actix_users")
            .build()
    ); // set up config for mongo connection

    let pool = Pool::builder()
        .max_size(16)
        .build(manager)
        .unwrap(); // instantiat connection pool 



    let serv_config = Config::get_config()
        .expect("Server configuration");

    info!("Starting server at {}", serv_config.host);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
           // .service(handlers::hello)
           // .service(handlers::user)
            .service(handlers::reguser)
            //.service(handlers::index)
            //.service(handlers::extract)
           // .route("/hey", web::get().to(handlers::manual_hello))
    })
    .bind(serv_config.host)?
    .run()
    .await?;

   Ok(())
}
