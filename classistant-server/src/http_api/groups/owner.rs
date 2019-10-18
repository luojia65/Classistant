use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};
use crate::app_api::{self, AppApi};
use crate::db::Database;
use crate::http_api::ErrorResponse;

#[derive(Deserialize)]
pub struct TransferRequest {
    api_version: String,
    group_id: u64,
    dest_user_id: u64,
}

#[derive(Serialize, Default)]
pub struct TransferResponse {}

pub fn transfer(
    id: Identity, 
    db: web::Data<Database>,
    params: web::Form<TransferRequest>,
) -> HttpResponse {
    if let AppApi::Api191017 = app_api::get(&params.api_version) {
        let user_id = identity_user_id!(id);
        match app_api::api_191017::group_transfer_owner(
            &db, 
            params.group_id, 
            user_id, 
            params.dest_user_id
        ) {
            Ok(()) => HttpResponse::Ok().json(TransferResponse {}),
            Err(crate::Error::OperatorUserNotInGroup) => forbidden!("user logged in is not in this group".to_string()),
            Err(crate::Error::DestUserNotInGroup) => forbidden!("dest user is not in this group".to_string()),
            Err(crate::Error::PermissionDenied) => forbidden!("permission denied".to_string()),
            Err(err) => internal!(err)
        }
    } else {
        invalid_api!()
    }
}
