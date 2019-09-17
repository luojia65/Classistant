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
    succeed: Vec<String>,
}

pub fn modify(db: web::Data<mysql::Pool>, info: web::Json<ModifyRequest>) -> HttpResponse {
    let mut succeed = Vec::new();
    if info.action != ACTION_MODIFY_REQUEST {
        return modify_failed(20, "wrong action type", succeed);
    }
    if info.uid == 0 {
        return modify_failed(10, "invalid `uid`: cannot be zero", succeed);
    }
    let map = if let serde_json::Value::Object(map) = &info.data { 
        map
    } else {
        return modify_failed(22, "invalid `data` field: object required", succeed);
    };
    let mut conn = match db.get_conn() {
        Ok(r) => r,
        Err(_) => return modify_failed(30, "failed to get connection from database", succeed),    
    };
    let mut stmt = match conn.prepare("PDataInsert(?, ?, ?, ?)") { 
        Ok(r) => r,
        Err(_) => return modify_failed(31, "failed to prepare statement", succeed),    
    };
    for (key, value) in map.iter() {
        let value = if let serde_json::Value::String(s) = value {
            s
        } else {
            return modify_failed(23, "value in `data` must be a base64 encoded string", succeed);
        };
        let value = match base64::decode(&value) {
            Ok(r) => r,
            Err(_) => return modify_failed(21, "failed to decode base64 value", succeed),    
        };
        let type_id: [u8; 16] = md5::compute(key).into();
        match stmt.execute((info.uid, info.gid, &type_id, value)) {
            Ok(_) => {},
            Err(_) => return modify_failed(32, "failed to execute statement", succeed),    
        };
        succeed.push(key.to_string());
    }

    // let value = match base64::decode(&info.hash) {
    //     Ok(r) => r,
    //     Err(_) => return login_failed(21, "failed to decode base64 value"),    
    // };
    HttpResponse::Ok().json(&info.data)
}

#[inline]
fn modify_failed<T: Into<String>>(id: u32, reason: T, succeed: Vec<String>) -> HttpResponse {
    HttpResponse::Ok().json(ModifyReply {
        action: ACTION_MODIFY_REPLY,
        return_id: id,
        failed_reason: Some(reason.into()),
        succeed
    })
}

