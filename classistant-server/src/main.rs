#[macro_use]
extern crate lazy_static;

use actix_web::{web, App, HttpServer};

mod auth;

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/api/v1/auth/init", web::get().to(auth::init))
            .route("/api/v1/auth/validate", web::get().to(auth::validate))
    })
    .bind("127.0.0.1:8000").expect("bind to port 8000")
    .run().expect("start server");
}
