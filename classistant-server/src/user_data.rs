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

#[derive(Serialize)]
pub struct ModifyReply {
    action: &'static str,
    return_id: u32,
}

pub fn modify(db: web::Data<mysql::Pool>, info: web::Json<ModifyRequest>) -> HttpResponse {
    if info.action != ACTION_MODIFY_REQUEST {
        return register_failed(20, "wrong action type");
    }
    
    HttpResponse::Ok().json(&info.data)
}

#[inline]
fn register_failed<T: Into<String>>(id: u32, reason: T) -> HttpResponse {
    HttpResponse::Ok().json(ModifyReply {
        action: ACTION_MODIFY_REPLY,
        return_id: id,
        // failed_reason: Some(reason.into()),
        // success_uid: None,
    })
}

