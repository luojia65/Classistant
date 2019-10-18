use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::app_api::{self, AppApi};
use crate::db::Database;
use crate::identity::IdentityInner;
use crate::site_config::SiteConfig;
use crate::http_api::ErrorResponse;

#[derive(Deserialize)]
pub struct LoginRequest {
    api_version: String,
    input: String,
    hash: String,
}

#[derive(Serialize, Default)]
pub struct LoginResponse {
    user_id: u64,
}

pub fn login(
    id: Identity, 
    cfg: web::Data<Arc<SiteConfig>>, 
    db: web::Data<Database>,
    params: web::Form<LoginRequest>,
) -> HttpResponse {
    if let AppApi::Api191017 = app_api::get(&params.api_version) {
        match app_api::api_191017::login_by_auth_id(&db, &params.input, &params.hash) {
            Ok(user_id) => {
                let id_inner = IdentityInner::new_uid(user_id, cfg.max_alive_secs);
                id.remember(id_inner.to_json_string().unwrap());
                HttpResponse::Ok().json(LoginResponse {
                    user_id,
                })
            }
            Err(crate::Error::UserNotExists) => forbidden!("user not exists".to_string()),
            Err(crate::Error::WrongPassword) => forbidden!("wrong password".to_string()),
            Err(err) => internal!(err)
        }
    } else {
        invalid_api!()
    }
}

#[derive(Deserialize)]
pub struct LogoutRequest {
    api_version: String,
}

#[derive(Serialize)]
pub struct LogoutResponse {}

pub fn logout(
    id: Identity, 
    params: web::Form<LogoutRequest>
) -> HttpResponse {
    if let AppApi::Api191017 = app_api::get(&params.api_version) {
        if id.identity().is_some() {
            id.forget();
        } else {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error_message: "not logged in".to_string()
            })
        }
        HttpResponse::Ok().json(LogoutResponse {})
    } else {
        invalid_api!()
    }
}
