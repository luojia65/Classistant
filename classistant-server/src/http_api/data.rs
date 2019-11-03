use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};
use crate::app_api::{self, AppApi};
use crate::db::Database;
use crate::http_api::ErrorResponse;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct GetBatchRequest {
    api_version: String,
    keys: Vec<String>,
}

#[derive(Serialize, Default)]
pub struct GetBatchResponse {
    ret: HashMap<String, (Vec<u8>, Vec<u8>)>,
}

pub fn get_batch(
    id: Identity, 
    db: web::Data<Database>,
    params: web::Form<GetBatchRequest>,
) -> HttpResponse {
    if let AppApi::Api191017 = app_api::get(&params.api_version) {
        let user_id = identity_user_id!(id);
        match app_api::api_191017::data_get_batch(
            &db, 
            user_id, 
            &params.keys.iter().map(|a| a.as_str()).collect::<Vec<_>>()
        ) {
            Ok(ret) => HttpResponse::Ok().json(GetBatchResponse { ret }),
            Err(err) => internal!(err)
        }
    } else {
        invalid_api!()
    }
}
