use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};
use crate::app_api::{self, AppApi};
use crate::db::Database;
use crate::http_api::ErrorResponse;

#[derive(Deserialize)]
pub struct CreateRequest {
    api_version: String,
}

#[derive(Serialize, Default)]
pub struct CreateResponse {
    group_id: u64
}

pub fn create(
    id: Identity, 
    db: web::Data<Database>, 
    params: web::Form<CreateRequest>
) -> HttpResponse {
    if let AppApi::Api191017 = app_api::get(&params.api_version) {
        let user_id = identity_user_id!(id);
        match app_api::api_191017::group_create(&db, user_id) {
            Ok(group_id) => 
                HttpResponse::Created().json(CreateResponse {
                    group_id
                }),
            Err(err) => internal!(err),
        }
    } else {
        invalid_api!()
    }
}

#[derive(Deserialize)]
pub struct DeleteRequest {
    api_version: String,
}

#[derive(Serialize, Default)]
pub struct DeleteResponse {}

pub fn delete(
    id: Identity, 
    db: web::Data<Database>, 
    info: web::Path<(u64,)>,
    params: web::Form<DeleteRequest>
) -> HttpResponse {
    if let AppApi::Api191017 = app_api::get(&params.api_version) {
        let user_id = identity_user_id!(id);
        match app_api::api_191017::group_delete(&db, info.0, user_id) {
            Ok(()) => HttpResponse::Ok().json(DeleteResponse {}),
            Err(crate::Error::GroupNotExists) => forbidden!("group not exists".to_string()),
            Err(crate::Error::PermissionDenied) => forbidden!("permission denied".to_string()),
            Err(err) => internal!(err),
        }
    } else {
        invalid_api!()
    }
}
