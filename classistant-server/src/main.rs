use actix_web::{web, App, HttpServer, HttpResponse};

mod auth;
mod db;

fn main() {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(clap::Arg::with_name("http-api")
            .short("a")
            .long("http-api")
            .value_name("HOST[:PORT]")
            .help("Bind API http server to HOST:PORT")
            .takes_value(true))
        .arg(clap::Arg::with_name("mysql")
            .long("mysql")
            .value_names(&["HOST", "PORT", "USER", "PWD", "DBNAME"])
            .help("Connect to MySQL server")
            .required(true)
            .takes_value(true))
        .get_matches();
    let api_bind_addr = matches.value_of("http-api").unwrap_or("127.0.0.1:8000");
    let mysql_addr: Vec<&str> = matches.values_of("mysql").unwrap().collect();
    println!("Using mysql database {}:{}", mysql_addr[0], mysql_addr[1]);
    let db = db::connect(mysql_addr);
    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .route("/api/{path}", web::get().to(|| HttpResponse::MethodNotAllowed().body("use POST")))
            .route("/api/v1.auth.register", web::post().to(auth::register))
    })
    .bind(api_bind_addr).expect("bind API server")
    .run().expect("start API server");
}
