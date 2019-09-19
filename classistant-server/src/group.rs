use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};

const ACTION_CREATE_REQUEST: &str = "v1.group.create.request";
const ACTION_CREATE_REPLY: &str = "v1.group.create.reply";
const ACTION_ALTER_REQUEST: &str = "v1.group.alter.request";
const ACTION_ALTER_REPLY: &str = "v1.group.alter.reply";

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

pub fn create(db: web::Data<mysql::Pool>, info: web::Json<CreateRequest>) -> HttpResponse {
    if info.action != ACTION_CREATE_REQUEST {
        return create_failed(20, "wrong action type");
    }
    if info.owner_uid == 0 {
        return create_failed(10, "owner's user id cannot be zero");
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
pub struct AlterRequest {
    action: String,
    op: AlterType,
    gid: u64,
    uid: u64,
}

#[derive(Deserialize)]
pub enum AlterType {
    Add,
    Remove,
}

#[derive(Serialize)]
pub struct AlterResponse {
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

pub fn alter(db: web::Data<mysql::Pool>, info: web::Json<AlterRequest>) -> HttpResponse {
    if info.action != ACTION_ALTER_REQUEST {
        return alter_failed(20, "wrong action type");
    }
    if info.uid == 0 {       
        return alter_failed(10, "zero uid");
    }
    if info.gid == 0 {
        return alter_failed(11, "zero gid");
    }
    let mut conn = match db.get_conn() {
        Ok(r) => r,
        Err(_) => return alter_failed(30, "failed to get connection from database"),    
    };
    let stmt = match info.op {
        AlterType::Add => conn.prepare("CALL PGroupMemberAdd(?, ?)"),
        AlterType::Remove => conn.prepare("CALL PGroupMemberRemove(?, ?)"),
    };
    let mut stmt = match stmt {
        Ok(r) => r,
        Err(_) => return alter_failed(31, "failed to prepare statement"),    
    };
    let result = match stmt.execute((info.gid, info.uid)) {
        Ok(r) => r,
        Err(_) => return alter_failed(32, "failed to execute statement"),    
    };
    let state = match result.affected_rows() {
        /* Add: User exists as normal user, not updated
           Remove: user not exists in record, do not need to remove */
        0 => 0,
        /* Add: User not exists, added new user to group
           Remove: User exists, marked user record as `invalid` */
        1 => 1,
        /* Add: User exists but is owner or moderator, updated its priv to 0
           Remove: impossible */
        2 => 2,
        _ => -1,
    };
    HttpResponse::Ok().json(AlterResponse {
        action: ACTION_ALTER_REPLY,
        return_id: 0,
        failed_reason: None,
        success_state: Some(state),
    })
}

#[inline]
fn alter_failed<T: Into<String>>(id: u32, reason: T) -> HttpResponse {
    HttpResponse::Ok().json(AlterResponse {
        action: ACTION_ALTER_REPLY,
        return_id: id,
        failed_reason: Some(reason.into()),
        success_state: None,
    })
}
