#![allow(unused_imports)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
//For tests
#[macro_use] extern crate diesel_migrations;

use actix::prelude::*;
use actix_files as fs;
use actix_web::middleware::{
    identity::{CookieIdentityPolicy, IdentityService},
    Logger,
};
use actix_web::{web, App, HttpServer};
use actix_session::{CookieSession, Session};
use chrono::Duration;
use diesel::{r2d2::ConnectionManager};
use dotenv::dotenv;

mod wiring;
mod errors;
mod schema;

mod users;

use crate::wiring::DbPool;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var( "RUST_LOG", "activue=debug,actix_web=info,actix_server=info",);
    std::env::set_var("RUST_BACKTRACE", "1");//XXX works only for panic! macro
    env_logger::init();
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = wiring::db_init(db_url);

    HttpServer::new(move || {
        // secret is a random minimum 32 bytes long base 64 string
        let secret: String = dotenv::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
        // let domain: String = dotenv::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(CookieSession::signed(secret.as_bytes()).secure(false))
            // .wrap(IdentityService::new(
            //     CookieIdentityPolicy::new(secret.as_bytes())
            //         .name("auth")
            //         .path("/")
            //         .domain(domain.as_str())
            //         .max_age_time(Duration::days(1))
            //         .secure(false), // this can only be true if you have https
            // ))
            .service( web::scope("/api") // everything under '/api/' route
                    .service( web::resource("/auth") // routes for authentication
                            .route(web::post().to_async(users::controllers::auth::login))
                            .route(web::delete().to(users::controllers::auth::logout))
                            .route(web::get().to_async(users::controllers::auth::get_me)),
                    )
                    // .service( web::resource("/register").route( 
                    //         web::post().to_async(users::controllers::register::register),
                    //     ),
                    // )
                    // .service( web::resource("/validate").route( 
                    //         web::post().to_async(users::controllers::register::register),
                    //     ),
                    // )
            )
            .service( web::scope("/register") // everything under '/register/' route
                  .service( web::resource("/request").route(
                      web::post().to_async(users::controllers::register::request)
                  ))
                  // route to validate registration
                  .service( web::resource("/{hashlink}/{email}/{expires_at}").route(
                          web::get().to_async(users::controllers::register::validate_link)
                  ))
                  .service( web::resource("/validate").route(
                          web::get().to_async(users::controllers::register::register)
                  ))
            )
            // serve static files
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8000")?
    .run()
}
