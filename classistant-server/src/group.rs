use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};

const ACTION_CREATE_REQUEST: &str = "v1.group.create.request";
const ACTION_CREATE_REPLY: &str = "v1.group.create.reply";

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
    #[serde(rename = "uid")]
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
    
    unimplemented!()
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