#[macro_export(local_inner_macros)]
macro_rules! get_app_api {
    ($params: ident) => {
if let Some(api) = crate::app_api::get(&$params.api_version) {
    api
} else {
    return actix_web::HttpResponse::NotFound().json(RegisterResponse {
        error_message: Some("Wrong api version")
    }) 
}
    };
}
