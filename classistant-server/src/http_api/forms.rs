use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::app_api;
use crate::db::Database;
use crate::http_api::ErrorResponse;

#[derive(Deserialize)]
pub struct CreateRequest {
    perm: HashMap<String, String>,
    content: HashMap<String, String>, // JSON string
    class: String,
    extra: String,
}

#[derive(Serialize, Default)]
pub struct CreateResponse {
    form_id: u64,
}

pub fn create(
    id: Identity, 
    db: web::Data<Database>,
    params: web::Json<CreateRequest>,
) -> HttpResponse {
    let user_id = identity_user_id!(id);
    let _ = user_id; // todo: verify if user_id could create `perm`
    let perm = serde_json::to_string(&params.perm).unwrap();
    let content = serde_json::to_string(&params.content).unwrap();
    let extra = match base64::decode(&params.extra) { 
        Ok(ans) => ans,
        Err(err) => return bad_request!(err) 
    };
    match app_api::api_191103::form_type_create(
        &db, 
        &perm,
        &content,
        &params.class,
        &extra
    ) {
        Ok(form_id) => {
            HttpResponse::Ok().json(CreateResponse {
                form_id
            }) 
        },
        Err(err) => internal!(err)
    }
}

#[derive(Serialize, Default)]
pub struct GetResponse {
    content: HashMap<String, String>,
}

pub fn get(
    id: Identity,
    db: web::Data<Database>,
    path: web::Path<(u64,)>
) -> HttpResponse {
    let form_id = path.0;
    let user_id = identity_user_id!(id);
    match app_api::api_191103::form_type_get(
        &db, 
        user_id,
        form_id
    ) {
        Ok(content) => {
            HttpResponse::Ok().json(GetResponse {
                content: todo!()
            }) 
        },
        Err(err) => internal!(err)
    }
}
