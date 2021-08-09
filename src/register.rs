use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Register {
   pub username: String,
   pub password: String,

}

impl Responder for Register {

    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
} 

