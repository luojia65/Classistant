use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};
use crate::identity::IdentityInner;

const ACTION_CREATE_REQUEST: &str = "v1.group.create.request";
const ACTION_CREATE_REPLY: &str = "v1.group.create.reply";
const ACTION_MODIFY_REQUEST: &str = "v1.group.modify-member.request";
const ACTION_MODIFY_REPLY: &str = "v1.group.modify-member.reply";
const ACTION_REMOVE_REQUEST: &str = "v1.group.remove-member.request";
const ACTION_REMOVE_REPLY: &str = "v1.group.remove-member.reply";

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
    #[serde(rename = "gid")]
    success_gid: Option<u64>,
}

pub fn create(id: Identity, db: web::Data<mysql::Pool>, info: web::Json<CreateRequest>) -> HttpResponse {
    if info.action != ACTION_CREATE_REQUEST {
        return create_failed(20, "wrong action type");
    }
    if info.owner_uid == 0 {
        return create_failed(10, "owner's user id cannot be zero");
    }
    let id = match id.identity() {
        Some(id) => match IdentityInner::from_json_str(&id) {
            Ok(id) => id,
            _ => return create_failed(40, "illegal identity"),
        },
        _ => return create_failed(41, "no identity exist"),
    };
    if id.uid() != info.owner_uid {
        return create_failed(42, "permission denied");
    }
    if id.is_expired() {
        return create_failed(43, "identity expired");
    }
    let mut conn = match db.get_conn() {
        Ok(r) => r,
        Err(_) => return create_failed(30, "failed to get connection from database"),    
    };
    let mut stmt = match conn.prepare("CALL PGroupCreate(?)") { 
        Ok(r) => r,
        Err(_) => return create_failed(31, "failed to prepare statement"),    
    };
    let mut ans_iter = match stmt.execute((&info.owner_uid,)) {
        Ok(r) => r,
        Err(_) => return create_failed(32, "failed to execute statement"),    
    };
    let ans = match ans_iter.next() {
        Some(Ok(r)) => r,
        None => return create_failed(33, "unexpected end of return rows"),
        Some(Err(_)) => return create_failed(34, "failed to iterate over answer rows"),
    };
    let group_id: u64 = match ans.get("group_id") {
        Some(r) => r,
        None => return create_failed(35, "no `group_id` row returned"),
    };
    HttpResponse::Ok().json(CreateResponse {
        action: ACTION_CREATE_REPLY,
        return_id: 0,
        failed_reason: None,
        success_gid: Some(group_id),
    })
}

#[inline]
fn create_failed<T: Into<String>>(id: u32, reason: T) -> HttpResponse {
    HttpResponse::Ok().json(CreateResponse {
        action: ACTION_CREATE_REPLY,
        return_id: id,
        failed_reason: Some(reason.into()),
        success_gid: None,
    })
}

#[derive(Deserialize)]
pub struct ModifyRequest {
    action: String,
    operator_uid: u64,
    gid: u64,
    uid: u64,
    new_priv: u64,
}

#[derive(Serialize)]
pub struct ModifyResponse {
    action: &'static str,
    #[serde(rename = "return")]
    return_id: u32,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    failed_reason: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    success_state: Option<i32>, 
}

pub fn modify_member(
    id: Identity, 
    db: web::Data<mysql::Pool>, 
    info: web::Json<ModifyRequest>
) -> HttpResponse {
    if info.action != ACTION_MODIFY_REQUEST {
        return modify_failed(20, "wrong action type");
    }
    if info.uid == 0 {       
        return modify_failed(10, "zero uid");
    }
    if info.gid == 0 {
        return modify_failed(11, "zero gid");
    }
    let id = match id.identity() {
        Some(id) => match IdentityInner::from_json_str(&id) {
            Ok(id) => id,
            _ => return create_failed(40, "illegal identity"),
        },
        _ => return create_failed(41, "no identity exist"),
    };
    if id.uid() != info.operator_uid {
        return create_failed(42, "permission denied");
    }
    if id.is_expired() {
        return create_failed(43, "identity expired");
    }
    let mut conn = match db.get_conn() {
        Ok(r) => r,
        Err(_) => return modify_failed(30, "failed to get connection from database"),    
    };
    let mut stmt = match conn.prepare("CALL PGroupMemberModify(?, ?, ?, ?)") {
        Ok(r) => r,
        Err(_) => return modify_failed(31, "failed to prepare statement"),    
    };
    let mut ans_iter = match stmt.execute((info.gid, info.uid, info.new_priv, info.operator_uid)) {
        Ok(r) => r,
        Err(_) => return modify_failed(32, "failed to execute statement"),    
    };
    let ans = match ans_iter.next() {
        Some(Ok(r)) => r,
        None => return modify_failed(33, "unexpected end of return rows"),
        Some(Err(_)) => return modify_failed(34, "failed to iterate over answer rows"),
    };
    let return_id: u64 = match ans.get("return_id") {
        Some(r) => r,
        None => return modify_failed(35, "no `return_id` row returned"),
    };
    match return_id {
        0 => {},
        1 => return modify_failed(44, "permission denied"),
        _ => return modify_failed(36, "invalid `return_id` value"),
    }
    let state = match ans_iter.affected_rows() {
        /* Add: User exists with same priv, not updated */
        0 => 0,
        /* Add: User not exists, added new user to group */
        1 => 1,
        /* User exists but old_priv != new_priv, set into new_priv*/
        2 => 2,
        _ => -1,
    };
    HttpResponse::Ok().json(ModifyResponse {
        action: ACTION_MODIFY_REPLY,
        return_id: 0,
        failed_reason: None,
        success_state: Some(state),
    })
}

#[inline]
fn modify_failed<T: Into<String>>(id: u32, reason: T) -> HttpResponse {
    HttpResponse::Ok().json(ModifyResponse {
        action: ACTION_MODIFY_REPLY,
        return_id: id,
        failed_reason: Some(reason.into()),
        success_state: None,
    })
}

#[derive(Deserialize)]
pub struct RemoveRequest {
    action: String,
    gid: u64,
    uid: u64,
}

#[derive(Serialize)]
pub struct RemoveResponse {
    action: &'static str,
    #[serde(rename = "return")]
    return_id: u32,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    failed_reason: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    success_state: Option<i32>, 
}


pub fn remove_member(db: web::Data<mysql::Pool>, info: web::Json<RemoveRequest>) -> HttpResponse {
    if info.action != ACTION_REMOVE_REQUEST {
        return remove_failed(20, "wrong action type");
    }
    if info.uid == 0 {       
        return remove_failed(10, "zero uid");
    }
    if info.gid == 0 {
        return remove_failed(11, "zero gid");
    }
    let mut conn = match db.get_conn() {
        Ok(r) => r,
        Err(_) => return remove_failed(30, "failed to get connection from database"),    
    };
    let mut stmt = match conn.prepare("CALL PGroupMemberRemove(?, ?)") {
        Ok(r) => r,
        Err(_) => return remove_failed(31, "failed to prepare statement"),    
    };
    let result = match stmt.execute((info.gid, info.uid)) {
        Ok(r) => r,
        Err(_) => return remove_failed(32, "failed to execute statement"),    
    };
    let state = match result.affected_rows() {
        /* Remove: User does not exists, do not need to remove */
        0 => 0,
        /* Remove: Use exists, performed remove process */
        1 => 1,
        _ => -1,
    };
    HttpResponse::Ok().json(RemoveResponse {
        action: ACTION_REMOVE_REPLY,
        return_id: 0,
        failed_reason: None,
        success_state: Some(state),
    })
}

#[inline]
fn remove_failed<T: Into<String>>(id: u32, reason: T) -> HttpResponse {
    HttpResponse::Ok().json(RemoveResponse {
        action: ACTION_REMOVE_REPLY,
        return_id: id,
        failed_reason: Some(reason.into()),
        success_state: None,
    })
}
