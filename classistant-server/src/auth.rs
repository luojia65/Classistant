use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};

const ACTION_REGISTER_REQUEST: &str = "v1.auth.register.request";
const ACTION_REGISTER_REPLY: &str = "v1.auth.register.reply";

#[derive(Deserialize)]
pub struct RegisterRequest {
    action: String,
    nickname: String,
    hash: String,
    locale: String,
}

#[derive(Serialize)]
pub struct RegisterResult {
    action: &'static str,
    #[serde(rename = "return")]
    return_id: u32,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    failed_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "uid")]
    success_uid: Option<u64>,
}

pub fn register(db: web::Data<mysql::Pool>, info: web::Json<RegisterRequest>) -> HttpResponse {
    if info.action != ACTION_REGISTER_REQUEST {
        return register_failed(20, "wrong action type");
    }
    let hash = match base64::decode(&info.hash) {
        Ok(r) => r,
        Err(_) => return register_failed(21, "failed to decode base64 hash"),    
    };
    if info.nickname.len() == 0 {       
        return register_failed(11, "empty nickname");
    }
    let mut all_numbers = true;
    for ch in info.nickname.chars() {
        if !ch.is_digit(10) {
            all_numbers = false;
            break;
        }
    }
    if all_numbers {
        return register_failed(12, "invalid nickname");
    }
    let _ = info.locale; // 
    let mut conn = match db.get_conn() {
        Ok(r) => r,
        Err(_) => return register_failed(30, "failed to get connection from database"),    
    };
    let mut stmt = match conn.prepare("CALL PUserRegister(?, ?)") {
        Ok(r) => r,
        Err(_) => return register_failed(31, "failed to prepare statement"),    
    };
    let mut ans_iter = match stmt.execute((&info.nickname, hash)) {
        Ok(r) => r,
        Err(_) => return register_failed(32, "failed to execute statement"),    
    };
    let ans = match ans_iter.next() {
        Some(Ok(r)) => r,
        None => return register_failed(33, "unexpected end of return rows"),
        Some(Err(_)) => return register_failed(34, "failed to iterate over answer rows"),
    };
    let return_id: u64 = match ans.get("return_id") {
        Some(r) => r,
        None => return register_failed(35, "no `return_id` row returned"),
    };
    if return_id != 0 {
        if return_id == 1 {
            return register_failed(10, "user nickname already taken")
        } else {
            return register_failed(36, "invalid `return_id` value")
        }
    }
    let uid: u64 = match ans.get("user_id") {
        Some(r) => r,
        None => return register_failed(37, "invalid `return_id` value")
    };
    HttpResponse::Ok().json(RegisterResult {
        action: ACTION_REGISTER_REPLY,
        return_id: 0,
        failed_reason: None,
        success_uid: Some(uid),
    })
}

#[inline]
fn register_failed<T: Into<String>>(id: u32, reason: T) -> HttpResponse {
    HttpResponse::Ok().json(RegisterResult {
        action: ACTION_REGISTER_REPLY,
        return_id: id,
        failed_reason: Some(reason.into()),
        success_uid: None,
    })
}
