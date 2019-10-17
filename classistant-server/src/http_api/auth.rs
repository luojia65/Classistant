use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct RegisterRequest {
    nickname: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    error_code: u32,
    message: &'static str,
}

pub fn register(
    db: web::Data<mysql::Pool>,
    form: web::Json<RegisterRequest>,
) -> HttpResponse {
    HttpResponse::Ok().json(RegisterResponse {
        error_code: 1,
        message: "Unimplemented"
    })
}

