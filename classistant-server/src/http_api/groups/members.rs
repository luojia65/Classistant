use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use serde::{Serialize, Deserialize};
use crate::app_api;
use crate::db::Database;
use crate::http_api::ErrorResponse;
use std::collections::HashMap;

// #[derive(Deserialize)]
// pub struct AddRequest {
//     group_id: u64,
//     dest_user_id: u64,
// }

// #[derive(Serialize, Default)]
// pub struct AddResponse {}

// pub fn add(
//     id: Identity, 
//     db: web::Data<Database>,
//     params: web::Form<AddRequest>,
// ) -> HttpResponse {
//     // let user_id = identity_user_id!(id);
//     // match app_api::api_191103::group_transfer_owner(
//     //     &db, 
//     //     params.group_id, 
//     //     user_id, 
//     //     params.dest_user_id
//     // ) {
//     //     Ok(()) => HttpResponse::Ok().json(AddResponse {}),
//     //     Err(crate::Error::OperatorUserNotInGroup) => forbidden!("user logged in is not in this group".to_string()),
//     //     Err(crate::Error::DestUserNotInGroup) => forbidden!("dest user is not in this group".to_string()),
//     //     Err(crate::Error::PermissionDenied) => forbidden!("permission denied".to_string()),
//     //     Err(err) => internal!(err)
//     // }
//     unimplemented!()
// }

#[derive(Deserialize)]
pub struct GetBatchRequest {}

#[derive(Serialize)]
pub struct GetBatchResponse {
    #[serde(flatten)]
    result: HashMap<String, String>
}

pub fn get_batch(
    id: Identity, 
    db: web::Data<Database>,
    params: web::Json<GetBatchRequest>,
) -> HttpResponse {
    let user_id = identity_user_id!(id);
    // match app_api::api_191103::data_get_batch(
    //     &db, 
    //     user_id, 
    //     params.keys.clone()
    // ) {
    //     Ok(ret) => {
    //         let mut resp = HashMap::new();
    //         for (k, (d, e)) in ret {
    //             let data_base64 = base64::encode(&d);
    //             let encryption_base64 = base64::encode(&e);
    //             resp.insert(k, [data_base64, encryption_base64]);
    //         }
    //         HttpResponse::Ok().json(resp) 
    //     },
    //     Err(err) => internal!(err)
    // }
    unimplemented!()
}