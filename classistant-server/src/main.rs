use actix_web::{web, App, HttpServer, HttpResponse};
use actix_identity::{CookieIdentityPolicy, IdentityService};

use pest_derive::Parser;
use pest::Parser;

use std::thread;
use std::sync::Arc;

#[macro_use]
mod macros;

mod auth_hash;
mod identity;
mod group;

mod site_config;
mod db;
mod http_api;
mod app_api;
mod result;

pub use result::{Error, Result};

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
    let db = db::connect_mysql(mysql_addr);
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
                    .secure(false))) // todo: https
                .route("/users", web::post().guard(header_191103!())
                    .to(http_api::users::register))
                // .route("/users/{}", web::delete().to(auth::unregister))
                .route("/sessions", web::post().guard(header_191103!())
                    .to(http_api::sessions::login))
                .route("/sessions", web::delete().guard(header_191103!())
                    .to(http_api::sessions::logout))
                .route("/data", web::get().guard(header_191103!())
                    .to(http_api::data::get_batch))
                .route("/data", web::put().guard(header_191103!())
                    .to(http_api::data::modify_batch))
                .route("/data", web::delete().guard(header_191103!())
                    .to(http_api::data::delete_batch))
                // .route("/users/{}/groups", web::get().to(group::get_by_user))
                .route("/groups", web::post().guard(header_191103!())
                    .to(http_api::groups::create))
                .route("/groups/{group_id}", web::delete().guard(header_191103!())
                    .to(http_api::groups::delete))
                // .route("/groups/{group_id}/members", web::get().to(group::members))
                .route("/groups/{group_id}/members/{user_id}", web::post().guard(header_191103!())
                    .to(http_api::groups::members::add))
                // .route("/groups/{group_id}/members/{user_id}", web::put().to(http_api::groups::members::modify))
                // .route("/groups/{group_id}/members/{user_id}", web::delete().to(http_api::groups::members::remove))
                // .route("/groups/{group_id}/owner", web::get().to(http_api::groups::owner::get))
                .route("/groups/{group_id}/owner", web::put().guard(header_191103!())
                    .to(http_api::groups::owner::transfer))
                // .route("/form", web::post().to(http_api::form::create))
                // .route("/form/{form_id}", web::get().to(http_api::form::get))
                // .route("/form/{form_id}", web::delete().to(http_api::form::delete))
                // .route("/form/{form_id}/fill/{user_id}", web::post().to(http_api::form::fill))
                .default_service(web::route().to(|| HttpResponse::NotFound()))
        })
        .bind(api_bind_addr).expect("bind API server")
        .run().expect("run API server");
    });
    println!("Successfully launched Classistant-Server!");
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        match CommandParser::parse(Rule::command, &buf.trim()) {
            Ok(mut pairs) => match pairs.next().map(|p| p.as_rule()) {
                Some(Rule::cmd_stop_head) => {
                    println!("Shutdown!");
                    std::process::exit(0);
                }, 
                Some(Rule::cmd_huaji_head) => {
                    println!("Huaji");
                },
                Some(Rule::EOI) => {},
                _ => eprintln!("unreachable expression, this is a bug!")
            },
            Err(e) => {
                eprintln!("err: <Console Input> {}", e);
            }
        }
    }
}

#[derive(Parser)]
#[grammar = "command_line.pest"]
struct CommandParser;

