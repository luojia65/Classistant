use actix_session::{CookieSession, Session};
use actix_web::{web, App, Error, HttpResponse, HttpServer};

fn auth_init(session: Session) -> Result<HttpResponse, Error> {
    session.set("public-key", "Wow")?;
    Ok(HttpResponse::Ok().body(format!(
        "{}",
        session.get::<String>("public-key")?.unwrap()
    )))
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .route("/api/v1/auth/init", web::get().to(auth_init))
    })
    .bind("127.0.0.1:8000").expect("bind to port 8000")
    .run().expect("start server");
}
