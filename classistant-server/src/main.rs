use actix_web::{web, App, HttpServer, HttpResponse};

mod auth;

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/api/{path}", web::get().to(|| HttpResponse::MethodNotAllowed()))
            .route("/api/v1.auth.register", web::post().to(auth::register))
    })
    .bind("127.0.0.1:8000").expect("bind to port 8000")
    .run().expect("start server");
}
