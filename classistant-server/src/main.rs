use actix_web::{web, App, HttpServer, HttpResponse};

use std::thread;

mod auth;
mod db;
mod data_user;
mod group;

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
    let api_bind_addr = matches.value_of("http-api").unwrap_or("127.0.0.1:8000").to_owned();
    let mysql_addr: Vec<&str> = matches.values_of("mysql").unwrap().collect();
    println!("Using MySQL database {}:{}", mysql_addr[0], mysql_addr[1]);
    let db = db::connect(mysql_addr);
    println!("Connected to MySQL!");
    thread::spawn(move || {
        println!("HTTP API starting at {}", api_bind_addr);
        HttpServer::new(move || {
            App::new()
                .data(db.clone())
                .route("/api/{path}", web::get().to(|| HttpResponse::MethodNotAllowed().body("use POST")))
                .route("/api/v1.auth.register", web::post().to(auth::register))
                .route("/api/v1.auth.login", web::post().to(auth::login))
                .route("/api/v1.user-data.modify", web::post().to(data_user::modify))
                .route("/api/v1.user-data.get", web::post().to(data_user::get))
                .route("/api/v1.group.create", web::post().to(group::create))
                .route("/api/v1.group.modify", web::post().to(group::modify))
        })
        .bind(api_bind_addr).expect("bind API server")
        .run().expect("run API server");
    });
    println!("Successfully launched Classistant-Server!");
    loop {}
}
