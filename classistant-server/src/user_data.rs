use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};

const ACTION_MODIFY_REQUEST: &str = "v1.user-data.modify.request";
const ACTION_MODIFY_REPLY: &str = "v1.user-data.modify.reply";

#[derive(Deserialize)]
pub struct ModifyRequest {
    action: String,
    uid: u64,
    data: serde_json::Value,
}

pub fn modify(db: web::Data<mysql::Pool>, info: web::Json<ModifyRequest>) -> HttpResponse {
    HttpResponse::Ok().json(&info.data)
}

