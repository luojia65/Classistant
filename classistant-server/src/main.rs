use actix_web::{web, App, HttpServer, HttpResponse};
use actix_identity::{CookieIdentityPolicy, IdentityService};

use std::thread;
use std::sync::Arc;

mod site_config;
mod auth;
mod identity;
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
        .arg(clap::Arg::with_name("max-alive-secs")
            .long("max-alive-secs")
            .value_name("SECONDS")
            .help("Maximum login session time in seconds")
            .takes_value(true))
        .get_matches();
    let api_bind_addr = matches.value_of("http-api").unwrap_or("127.0.0.1:8000").to_owned();
    let mysql_addr: Vec<&str> = matches.values_of("mysql").unwrap().collect();
    let max_alive_secs: u64 = matches.value_of("max-alive-secs").unwrap_or("86400").parse()
        .expect("numberic maximum alive seconds for clients");
    let site_cfg = Arc::new(site_config::SiteConfig {
        max_alive_secs
    });
    println!("Using MySQL database {}:{}", mysql_addr[0], mysql_addr[1]);
    let db = db::connect(mysql_addr);
    println!("Connected to MySQL!");
    thread::spawn(move || {
        let site_cfg = site_cfg.clone();
        println!("HTTP API starting at {}", api_bind_addr);
        HttpServer::new(move || {
            App::new()
                .data(db.clone())
                .data(site_cfg.clone())
                .wrap(IdentityService::new(CookieIdentityPolicy::new(&[0; 64]) 
                    .name("classistant-identity")
                    .max_age(max_alive_secs as i64)
                    .secure(false)))
                .route("/api/{path}", web::get().to(|| HttpResponse::MethodNotAllowed().body("use POST")))
                .route("/api/v1.auth.register", web::post().to(auth::register))
                .route("/api/v1.auth.login", web::post().to(auth::login))
                .route("/api/v1.auth.logout", web::post().to(auth::logout))
                .route("/api/v1.user-data.modify", web::post().to(data_user::modify))
                .route("/api/v1.user-data.get", web::post().to(data_user::get))
                .route("/api/v1.group.create", web::post().to(group::create))
                .route("/api/v1.group.modify-member", web::post().to(group::modify_member))
                .route("/api/v1.group.remove-member", web::post().to(group::remove_member))
                .default_service(web::route().to(|| HttpResponse::NotFound()))
        })
        .bind(api_bind_addr).expect("bind API server")
        .run().expect("run API server");
    });
    println!("Successfully launched Classistant-Server!");
    loop {
        // todo: there should be a console command line process
    }
}
