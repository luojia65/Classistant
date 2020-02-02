use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};
use crate::app_api;
use crate::db::Database;
use crate::http_api::ErrorResponse;

#[derive(Deserialize)]
pub struct CreateRequest {
    perm: String,
    content: String, // JSON string
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
    let extra = match base64::decode(&params.extra) { 
        Ok(ans) => ans,
        Err(err) => return bad_request!(err) 
    };
    match app_api::api_191103::form_type_create(
        &db, 
        &params.perm,
        &params.content,
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
