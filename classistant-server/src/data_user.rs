use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};
use crate::identity::IdentityInner;

const ACTION_MODIFY_REQUEST: &str = "v1.user-data.modify.request";
const ACTION_MODIFY_REPLY: &str = "v1.user-data.modify.reply";
const ACTION_GET_REQUEST: &str = "v1.user-data.get.request";
const ACTION_GET_REPLY: &str = "v1.user-data.get.reply";

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

pub fn modify(id: Identity, db: web::Data<mysql::Pool>, info: web::Json<ModifyRequest>) -> HttpResponse {
    let mut state_map = serde_json::Map::new();
    if info.action != ACTION_MODIFY_REQUEST {
        return modify_failed(20, "wrong action type", state_map);
    }
    if info.uid == 0 {
        return modify_failed(10, "invalid `uid`: cannot be zero", state_map);
    }
    let id = match id.identity() {
        Some(id) => match IdentityInner::from_json_str(&id) {
            Ok(id) => id,
            _ => return modify_failed(40, "illegal identity", state_map),
        },
        _ => return modify_failed(41, "no identity exist", state_map),
    };
    if id.uid() != info.uid {
        return modify_failed(42, "permission denied", state_map);
    }
    if id.is_expired() {
        return modify_failed(43, "identity expired", state_map);
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
#[derive(Deserialize)]
pub struct GetRequest {
    action: String,
    uid: u64,
    gid: u64,
    keys: serde_json::Value,
}

#[derive(Serialize)]
pub struct GetReply {
    action: &'static str,
    #[serde(rename = "return")]
    return_id: u32,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    failed_reason: Option<String>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    success_data: serde_json::Value,
}

pub fn get(id: Identity, db: web::Data<mysql::Pool>, info: web::Json<GetRequest>) -> HttpResponse {
    let mut data_map = serde_json::Map::new();
    if info.action != ACTION_GET_REQUEST {
        return get_failed(20, "wrong action type");
    }
    if info.uid == 0 {
        return get_failed(10, "invalid `uid`: cannot be zero");
    }
    let id = match id.identity() {
        Some(id) => match IdentityInner::from_json_str(&id) {
            Ok(id) => id,
            _ => return get_failed(40, "illegal identity"),
        },
        _ => return get_failed(41, "no identity exist"),
    };
    if id.uid() != info.uid {
        return get_failed(42, "permission denied");
    }
    if id.is_expired() {
        return get_failed(43, "identity expired");
    }
    let array = if let serde_json::Value::Array(array) = &info.keys { 
        array
    } else {
        return get_failed(22, "invalid `keys` field: array required");
    };
    let mut conn = match db.get_conn() {
        Ok(r) => r,
        Err(_) => return get_failed(30, "failed to get connection from database"),    
    };
    let mut stmt = match conn.prepare("CALL PDataUGet(?, ?, ?)") { 
        Ok(r) => r,
        Err(_) => return get_failed(31, "failed to prepare statement"),    
    };
    for key in array.iter() {
        let key = if let serde_json::Value::String(s) = key { 
            s 
        } else { 
            return get_failed(23, "element in array `keys` must be strings"); 
        };
        let type_id: [u8; 16] = md5::compute(key).into();
        let mut ans_iter = match stmt.execute((info.uid, info.gid, &type_id)) {
            Ok(r) => r,
            Err(_) => return get_failed(32, "failed to execute statement"),    
        };
        let ans = match ans_iter.next() {
            Some(Ok(r)) => r,
            None => {
                data_map.insert(key.to_string(), serde_json::Value::Null);
                continue
            },
            Some(Err(_)) => return get_failed(34, "failed to iterate over answer rows"),
        };
        let value: Vec<u8> = match ans.get("data") {
            Some(r) => r,
            None => return get_failed(35, "no `data` row returned"),
        };
        let value = base64::encode(&value);
        data_map.insert(key.to_string(), serde_json::Value::String(value));
    }
    HttpResponse::Ok().json(GetReply {
        action: ACTION_GET_REPLY,
        return_id: 0,
        failed_reason: None,
        success_data: serde_json::Value::Object(data_map),
    })
}

#[inline]
fn get_failed<T: Into<String>>(id: u32, reason: T) -> HttpResponse {
    HttpResponse::Ok().json(GetReply {
        action: ACTION_GET_REPLY,
        return_id: id,
        failed_reason: Some(reason.into()),
        success_data: serde_json::Value::Null,
    })
}
