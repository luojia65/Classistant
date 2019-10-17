use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::get_app_api;
use crate::db;

#[derive(Deserialize)]
pub struct RegisterRequest {
    api_version: String,
    nickname: String,
    hash: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<&'static str>,
}

pub fn register(
    db: web::Data<db::Database>,
    params: web::Json<RegisterRequest>,
) -> HttpResponse {
    let api = get_app_api!(params);
    HttpResponse::Ok().json(RegisterResponse {
        error_message: None
    })
}

