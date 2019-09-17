use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};

const ACTION_MODIFY_REQUEST: &str = "v1.user-data.modify.request";
const ACTION_MODIFY_REPLY: &str = "v1.user-data.modify.reply";

#[derive(Deserialize)]
pub struct ModifyRequest {
    action: String,
    uid: u64,
    gid: u64,
    data: serde_json::Value,
}

#[derive(Serialize)]
pub struct ModifyReply {
    action: &'static str,
    #[serde(rename = "return")]
    return_id: u32,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    failed_reason: Option<String>,
}

pub fn modify(db: web::Data<mysql::Pool>, info: web::Json<ModifyRequest>) -> HttpResponse {
    if info.action != ACTION_MODIFY_REQUEST {
        return modify_failed(20, "wrong action type");
    }
    if info.uid == 0 {
        return modify_failed(10, "invalid `uid`: cannot be zero");
    }
    let map = if let serde_json::Value::Object(map) = &info.data { 
        map
    } else {
        return modify_failed(22, "invalid `data` field: object required");
    };
    let mut conn = match db.get_conn() {
        Ok(r) => r,
        Err(_) => return modify_failed(30, "failed to get connection from database"),    
    };
    let mut stmt = match conn.prepare("") { // todo
        Ok(r) => r,
        Err(_) => return modify_failed(31, "failed to prepare statement"),    
    };

    // let value = match base64::decode(&info.hash) {
    //     Ok(r) => r,
    //     Err(_) => return login_failed(21, "failed to decode base64 value"),    
    // };
    HttpResponse::Ok().json(&info.data)
}

#[inline]
fn modify_failed<T: Into<String>>(id: u32, reason: T) -> HttpResponse {
    HttpResponse::Ok().json(ModifyReply {
        action: ACTION_MODIFY_REPLY,
        return_id: id,
        failed_reason: Some(reason.into()),
        // success_uid: None,
    })
}

