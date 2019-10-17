#[macro_export(local_inner_macros)]
macro_rules! invalid_api {
    ($T: ident) => {
        actix_web::HttpResponse::NotImplemented().json($T {
            error_message: Some("wrong api version".to_string()),
            .. Default::default()
        }) 
    };
}
