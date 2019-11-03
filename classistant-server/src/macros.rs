#[macro_export(local_inner_macros)]
macro_rules! identity_user_id {
    ($id: expr) => {{
        let id = if let Some(id) = $id.identity() { id } else {
            return actix_web::HttpResponse::Unauthorized().json(ErrorResponse {
                error_message: "not logged in".to_string()
            })
        };
        let id = match crate::identity::IdentityInner::from_json_str(&id) {
            Ok(id) => id,
            Err(e) => 
                return actix_web::HttpResponse::Forbidden().json(ErrorResponse {
                    error_message: std::format!("illegal identity: {}", e)
                })
        };
        if id.is_expired() {
            return actix_web::HttpResponse::Forbidden().json(ErrorResponse {
                error_message: std::format!("identity expired")
            })
        }
        id.user_id()
    }};
}

#[macro_export(local_inner_macros)]
macro_rules! invalid_api {
    () => {
        actix_web::HttpResponse::NotImplemented().json(crate::http_api::ErrorResponse {
            error_message: "incorrect api version".to_string(),
            .. Default::default()
        }) 
    };
}

#[macro_export(local_inner_macros)]
macro_rules! forbidden {
    ($msg: expr) => {
        actix_web::HttpResponse::Forbidden().json(crate::http_api::ErrorResponse {
            error_message: $msg,
        })
    };
}

#[macro_export(local_inner_macros)]
macro_rules! internal {
    ($err: expr) => {
        actix_web::HttpResponse::InternalServerError().json(crate::http_api::ErrorResponse {
            error_message: std::format!("internal error: {}", $err),
        })
    };
}

#[macro_export(local_inner_macros)]
macro_rules! bad_request {
    ($err: expr) => {
        actix_web::HttpResponse::BadRequest().json(crate::http_api::ErrorResponse {
            error_message: std::format!("bad request: {}", $err),
        })
    };
}
