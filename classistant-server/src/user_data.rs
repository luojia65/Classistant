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
    state: serde_json::Value,
}

pub fn modify(db: web::Data<mysql::Pool>, info: web::Json<ModifyRequest>) -> HttpResponse {
    let mut state_map = serde_json::Map::new();
    if info.action != ACTION_MODIFY_REQUEST {
        return modify_failed(20, "wrong action type", state_map);
    }
    if info.uid == 0 {
        return modify_failed(10, "invalid `uid`: cannot be zero", state_map);
    }
    let map = if let serde_json::Value::Object(map) = &info.data { 
        map
    } else {
        return modify_failed(22, "invalid `data` field: object required", state_map);
    };
    let mut conn = match db.get_conn() {
        Ok(r) => r,
        Err(_) => return modify_failed(30, "failed to get connection from database", state_map),    
    };
    let mut stmt = match conn.prepare("CALL PDataUInsert(?, ?, ?, ?)") { 
        Ok(r) => r,
        Err(_) => return modify_failed(31, "failed to prepare statement", state_map),    
    };
    for (key, value) in map.iter() {
        let value = if let serde_json::Value::String(s) = value {
            s
        } else {
            return modify_failed(23, "value in `data` must be a base64 encoded string", state_map);
        };
        let value = match base64::decode(&value) {
            Ok(r) => r,
            Err(_) => return modify_failed(21, "failed to decode base64 value", state_map),    
        };
        let type_id: [u8; 16] = md5::compute(key).into();
        let result = match stmt.execute((info.uid, info.gid, &type_id, value)) {
            Ok(r) => r,
            Err(_) => return modify_failed(32, "failed to execute statement", state_map),    
        };
        let state = match result.affected_rows() {
            0 /* Same value exists */ => 0,
            1 /* Insert success */ => 1,
            2 /* Update success */=> 2,
            _ => -1,
        };
        state_map.insert(key.to_string(), serde_json::Value::Number(state.into()));
    }
    HttpResponse::Ok().json(ModifyReply {
        action: ACTION_MODIFY_REPLY,
        return_id: 0,
        failed_reason: None,
        state: serde_json::Value::Object(state_map)
    })
}

#[inline]
fn modify_failed<T: Into<String>>(
    id: u32, 
    reason: T, 
    state_map: serde_json::Map<String, serde_json::Value>
) -> HttpResponse {
    HttpResponse::Ok().json(ModifyReply {
        action: ACTION_MODIFY_REPLY,
        return_id: id,
        failed_reason: Some(reason.into()),
        state: serde_json::Value::Object(state_map)
    })
}

