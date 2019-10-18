use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::app_api::{self, AppApi};
use crate::db;
use crate::identity::IdentityInner;
use crate::site_config::SiteConfig;

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
    params: web::Form<RegisterRequest>,
) -> HttpResponse {
    if let AppApi::Api191017 = app_api::get(&params.api_version) {
        match app_api::api_191017::register_user_by_nickname(&db, &params.nickname, &params.hash) {
            Ok(user_id) => 
                HttpResponse::Created().json(RegisterResponse {
                    user_id: Some(user_id),
                    error_message: None,
                }),
            Err(crate::Error::UserAlreadyExists) => 
                HttpResponse::Forbidden().json(RegisterResponse {
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

#[derive(Deserialize)]
pub struct LoginRequest {
    api_version: String,
    input: String,
    hash: String,
}

#[derive(Serialize, Default)]
pub struct LoginResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}

pub fn login(
    id: Identity, 
    cfg: web::Data<Arc<SiteConfig>>, 
    db: web::Data<db::Database>,
    params: web::Form<LoginRequest>,
) -> HttpResponse {
    if let AppApi::Api191017 = app_api::get(&params.api_version) {
        match app_api::api_191017::login_by_auth_id(&db, &params.input, &params.hash) {
            Ok(user_id) => {
                let id_inner = IdentityInner::new_uid(user_id, cfg.max_alive_secs);
                id.remember(id_inner.to_json_string().unwrap());
                HttpResponse::Ok().json(LoginResponse {
                    user_id: Some(user_id),
                    error_message: None,
                })
            }
            Err(crate::Error::UserNotExists) => 
                HttpResponse::Forbidden().json(LoginResponse {
                    user_id: None,
                    error_message: Some("user not exists".to_string()),
                }),
            Err(crate::Error::WrongPassword) => 
                HttpResponse::Forbidden().json(LoginResponse {
                    user_id: None,
                    error_message: Some("wrong password".to_string()),
                }),
            Err(err) => 
                HttpResponse::InternalServerError().json(LoginResponse {
                    user_id: None,
                    error_message: Some(format!("internal error: {}", err)),
                }),
        }
    } else {
        invalid_api!(RegisterResponse)
    }
}

