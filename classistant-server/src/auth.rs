use actix_session::Session;
use actix_web::{web, Responder, HttpRequest, HttpResponse};
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
    success: bool,
    #[serde(flatten)]
    detailed: Detailed,
}

#[derive(Serialize)]
pub enum Detailed {
    Success { uid: String },
    Failed { reason: String },
}

pub fn register(info: web::Json<RegisterRequest>) -> impl Responder {
    let resp = RegisterResult {
        action: ACTION_REGISTER_REPLY,
        success: false,
        detailed: Detailed::Failed { reason: String::from("滑稽，滑稽") }
    };
    let ans = match serde_json::to_string(&resp) {
        Ok(r) => r,
        Err(e) => {
            let e_string = format!("{}", e).escape_unicode().to_string();
            format!("{{\"action\":{},\"success\":false,\"reason\":\"{}\"}}", 
                ACTION_REGISTER_REPLY.escape_unicode().to_string(),
                e_string
            )
        }
    };
    format!("{}", ans)
}

// #[derive(Serialize, Deserialize)]
// struct ValidateResult {
//     code: u32,
//     description: &'static str
// }

// pub fn validate(_session: Session) -> impl Responder {
//     HttpResponse::Ok().json(ValidateResult {
//         code: 0,
//         description: "Login succeeded"
//     })
// }
