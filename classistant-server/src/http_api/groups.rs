use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::app_api::{self, AppApi};
use crate::db::Database;
use crate::site_config::SiteConfig;
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
        unimplemented!()
    } else {
        invalid_api!()
    }
}
