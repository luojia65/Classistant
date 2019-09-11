use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};

const ACTION_REGISTER_REQUEST: &str = "v1.auth.register.request";
const ACTION_REGISTER_REPLY: &str = "v1.auth.register.reply";
const ACTION_LOGIN_REQUEST: &str = "v1.auth.login.request";
const ACTION_LOGIN_REPLY: &str = "v1.auth.login.reply";

#[derive(Deserialize)]
pub struct RegisterRequest {
    action: String,
    nickname: String,
    hash: String,
    locale: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
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
        None => return register_failed(37, "invalid `user_id` value")
    };
    HttpResponse::Ok().json(RegisterResponse {
        action: ACTION_REGISTER_REPLY,
        return_id: 0,
        failed_reason: None,
        success_uid: Some(uid),
    })
}

#[inline]
fn register_failed<T: Into<String>>(id: u32, reason: T) -> HttpResponse {
    HttpResponse::Ok().json(RegisterResponse {
        action: ACTION_REGISTER_REPLY,
        return_id: id,
        failed_reason: Some(reason.into()),
        success_uid: None,
    })
}

#[derive(Deserialize)]
pub struct LoginRequest {
    action: String,
    input: String,
    hash: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    action: &'static str,
    #[serde(rename = "return")]
    return_id: u32,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    failed_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "uid")]
    success_uid: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nickname")]
    success_nickname: Option<String>,
}

pub fn login(db: web::Data<mysql::Pool>, info: web::Json<LoginRequest>) -> HttpResponse { 
    if info.action != ACTION_LOGIN_REQUEST {
        return login_failed(20, "wrong action type");
    }
    let mut all_numbers = true;
    for ch in info.input.chars() {
        if !ch.is_digit(10) {
            all_numbers = false;
            break;
        }
    }
    if all_numbers {
        match info.input.parse::<u64>() {
            Err(_) | Ok(0) => return login_failed(10, "invalid number user id"),
            _ => {}
        }
    }
    let hash = match base64::decode(&info.hash) {
        Ok(r) => r,
        Err(_) => return login_failed(21, "failed to decode base64 hash"),    
    };
    let mut conn = match db.get_conn() {
        Ok(r) => r,
        Err(_) => return login_failed(30, "failed to get connection from database"),    
    };
    let stmt = if all_numbers {
        conn.prepare("CALL PUserLoginById(?, ?)")
    } else {
        conn.prepare("CALL PUserLoginByNickname(?, ?)")
    };
    let mut stmt = match stmt {
        Ok(r) => r,
        Err(_) => return login_failed(31, "failed to prepare statement"),    
    };
    let mut ans_iter = match stmt.execute((&info.input, hash)) {
        Ok(r) => r,
        Err(_) => return login_failed(32, "failed to execute statement"),    
    };
    let ans = match ans_iter.next() {
        Some(Ok(r)) => r,
        None => return login_failed(33, "unexpected end of return rows"),
        Some(Err(_)) => return login_failed(34, "failed to iterate over answer rows"),
    };
    let return_id: u64 = match ans.get("return_id") {
        Some(r) => r,
        None => return login_failed(35, "no `return_id` row returned"),
    };
    match return_id {
        0 => {},
        1 => return login_failed(11, "wrong password"),
        2 => return login_failed(12, "user of this id cannot be found"),
        3 => return login_failed(13, "user of this nickname cannot be found"),
        _ => return login_failed(36, "invalid `return_id` value"),
    }
    let user_id: u64 = match ans.get("user_id") {
        Some(r) => r,
        None => return register_failed(37, "invalid `user_id` value")
    };
    let nickname: String = match ans.get("nickname") {
        Some(r) => r,
        None => return register_failed(38, "invalid `nickname` value")
    };
    HttpResponse::Ok().json(LoginResponse {
        action: ACTION_LOGIN_REPLY,
        return_id: 0,
        failed_reason: None,
        success_uid: Some(user_id),
        success_nickname: Some(nickname)
    })
}   

#[inline]
fn login_failed<T: Into<String>>(id: u32, reason: T) -> HttpResponse {
    HttpResponse::Ok().json(LoginResponse {
        action: ACTION_LOGIN_REPLY,
        return_id: id,
        failed_reason: Some(reason.into()),
        success_uid: None,
        success_nickname: None,
    })
}
