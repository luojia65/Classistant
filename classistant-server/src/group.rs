use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};

const ACTION_CREATE_REQUEST: &str = "v1.group.create.request";

#[derive(Deserialize)]
pub struct CreateRequest {
    action: String,
    owner_uid: u64,
}

#[derive(Serialize)]
pub struct CreateResponse {
    action: &'static str,
    #[serde(rename = "return")]
    return_id: u32,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    failed_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "uid")]
    success_gid: Option<u64>,
}


pub fn create(db: web::Data<mysql::Pool>, info: web::Json<CreateRequest>) -> HttpResponse {
    unimplemented!()
}
