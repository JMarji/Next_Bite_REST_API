use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::error;
use r2d2::Pool;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager, VerifyPeer};
//use mongodb::db::{Database, ThreadedDatabase};

use crate::register::Register;

#[path = "register.rs"]
mod register;

// end of preprocessor directives ... start of handler code below


#[post("/reguser")] //post req for enpoint /reg
async fn reguser(form: web::Form<Register>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Welcome {}, password {}",
        form.username, form.password
    ))
}

/*
#[get("/")] // get request for endpoint /
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}
*/

/*
#[get("/user")] // get req for endpoint /user
async fn user() -> impl Responder {
    HttpResponse::Ok().body("users.json")
}
*/

// #[post("/echo")] // post req for endpoint /echo
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// handler for get req with out using macro endpoint /manual_hello
/*
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
*/


/*
#[get("/json")]
async fn index() -> impl Responder {
    Register {
        username: "user".to_string(),
        password: "password".to_string(),
    }
}
*/

/*
#[get("/users/{username}/{password}")]
async fn extract(
    register: web::Path<Register>,
    pool: web::Data<Pool<MongodbConnectionManager>>,
) -> impl Responder {
    format!(
        "Welcome {}, your password is {}",
        register.username, register.password
    )
}
*/
    // open connection to mongo db
/*
    let doc = doc! {
        "username": register.username,
        "password": register.password,
        "age": register.age,
    };

    let conn = pool.get().unwrap();


    let result = conn.collections("test").findOne()
*/
