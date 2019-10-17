use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::db;
use crate::app_api::{self, AppApi};

#[derive(Deserialize)]
pub struct RegisterRequest {
    api_version: String,
    nickname: String,
    hash: String,
}

#[derive(Serialize, Default)]
pub struct RegisterResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}

pub fn register(
    db: web::Data<db::Database>,
    params: web::Json<RegisterRequest>,
) -> HttpResponse {
    if let AppApi::Api191017 = app_api::get(&params.api_version) {
        match app_api::api_191017::register_user_by_nickname(&db, &params.nickname, &params.hash) {
            Ok(Some(user_id)) => 
                HttpResponse::Ok().json(RegisterResponse {
                    user_id: Some(user_id),
                    error_message: None,
                }),
            Ok(None) => 
                HttpResponse::Ok().json(RegisterResponse {
                    user_id: None,
                    error_message: Some("user already exists".to_string()),
                }),
            Err(err) => 
                HttpResponse::InternalServerError().json(RegisterResponse {
                    user_id: None,
                    error_message: Some(format!("internal error: {}", err)),
                }),
        }
    } else {
        invalid_api!(RegisterResponse)
    }
}

