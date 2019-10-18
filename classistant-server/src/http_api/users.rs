use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::app_api::{self, AppApi};
use crate::db::Database;

#[derive(Deserialize)]
pub struct RegisterRequest {
    api_version: String,
    nickname: String,
    hash: String,
}

#[derive(Serialize, Default)]
pub struct RegisterResponse {
    user_id: u64,
}

pub fn register(
    db: web::Data<Database>,
    params: web::Form<RegisterRequest>,
) -> HttpResponse {
    if let AppApi::Api191017 = app_api::get(&params.api_version) {
        match app_api::api_191017::register_user_by_nickname(&db, &params.nickname, &params.hash) {
            Ok(user_id) => 
                HttpResponse::Created().json(RegisterResponse {
                    user_id
                }),
            Err(crate::Error::UserAlreadyExists) => forbidden!("user already exists".to_string()),
            Err(err) => internal!(err)
        }
    } else {
        invalid_api!()
    }
}
