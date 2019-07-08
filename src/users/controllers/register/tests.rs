use crate::users;
use actix_web::{web, test, http, App};
use actix_http::HttpService;
use actix_http_test::TestServer;
use actix_session::{CookieSession, Session};
use chrono::{NaiveDate, NaiveDateTime};
use dotenv::dotenv;
use std::time::Duration;
use futures::future::Future;
use super::CommandResult;

use diesel::prelude::*;
use crate::schema::users::dsl;
use crate::users::models::{SlimUser, User, NewUser};

#[test]
fn test_request() {
    dotenv().ok();
    let mut srv = TestServer::new( || {
        let pool = crate::wiring::test_conn_init();
        //Insert test data 
        let conn = &pool.get().unwrap();
        let user = NewUser::with_details(String::from("login"), String::from("email@toto.fr"), String::from("password"));
        diesel::insert_into(dsl::users).values(&user)
            .execute(conn).expect("Error populating test database");

        HttpService::new(
            App::new().data(pool.clone()).service(
                web::resource("/register/request").route(
                    web::post().to_async(users::controllers::register::request)
                )
            )
        )
    });

    //==== Test request
    let form = super::RequestForm { email:  String::from("email2@toto.fr") };

    let req = srv.post("/register/request")
        .timeout(Duration::new(15, 0));
        // .header( http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"),);

    let mut response = srv.block_on(req.send_form(&form)).unwrap();
    // println!("{:#?}", response);
    assert!(response.status().is_success());
    let result: CommandResult = response.json().wait().expect("Could not parse json"); 
    assert!(result.success);

    //======== Test request with already registered email
    let existing_user = super::RequestForm {
        email: String::from("email@toto.fr"),
    };
    let req = srv.post("/register/request")
        .timeout(Duration::new(15, 0));

    let mut response = srv.block_on(req.send_form(&existing_user)).unwrap();
    assert!(response.status().is_success());
    let result: CommandResult = response.json().wait().expect("Could not parse json"); 
    assert!(!result.success);
    assert_eq!(Some(String::from("Email already taken")), result.error);
}

use regex::Regex;

#[test]
fn test_validate() {
    dotenv().ok();
    let mut srv = TestServer::new( move || {
        let pool = crate::wiring::test_conn_init();
        //Insert test data 
        let conn = &pool.get().unwrap();
        let user = NewUser::with_details(String::from("login"), String::from("email@toto.fr"), String::from("password"));
        // Batch don't work with Sqlite 
        // diesel::insert_into(dsl::users).values(&vec![user, user_expired])
            // .execute(conn).expect("Error populating test database");
        diesel::insert_into(dsl::users).values(&user)
            .execute(conn).expect("Error populating test database");

        HttpService::new(
            App::new().data(pool.clone())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service( web::resource("/register/request").route( // To test insertions 
                web::post().to_async(users::controllers::register::request)
            ))
            .service( web::resource("/register/{hashlink}/{login}/{expires_at}").route(
                web::get().to_async(users::controllers::register::validate_link)
            ))
            .service( web::resource("/register/validate").route( 
                web::post().to_async(users::controllers::register::register),
            ))
        )
    });

    // ================ Good link
    //
    let email = "email@test.fr";

    // 1. Mock register request
    let req = register_link_mock(&mut srv, email, email, false);
    // 2. Validate link
    let mut response = srv.block_on(req.send()).unwrap();
    assert!(response.status().is_success());
    // println!("response : {:#?}", response);
    let result: CommandResult = response.json().wait().expect("Could not parse json"); 
    // println!("err: {}", result.error.unwrap_or(String::from("none")));
    assert!(result.success);
    // 3. Finish registration with user data
    let mut req: awc::ClientRequest = srv.post("/register/validate").timeout(Duration::new(15, 0));
    req = keep_session(response, req);
    let form = super::ValidateForm { username:  String::from("username"), password: String::from("passwd") };
    let mut response = srv.block_on(req.send_form(&form)).unwrap();
    assert!(response.status().is_success());
    let result: CommandResult = response.json().wait().expect("Could not parse json"); 
    assert!(result.success);

    // ----------- Registering with same email should now fail
    let form_request = super::RequestForm { email:  String::from(email) };
    let req_request = srv.post("/register/request").timeout(Duration::new(15, 0));
    let mut response = srv.block_on(req_request.send_form(&form_request)).unwrap();
    // println!("response : {:#?}", response);
    assert!(response.status().is_success());
    let result: CommandResult = response.json().wait().expect("Could not parse json"); 
    assert!(!result.success);
    assert_eq!(Some(String::from("Email already taken")), result.error);

    // ----------- Registering with same username should now fail
    // 1. Mock register request
    let email = "emailNewName@test.fr";
    let req = register_link_mock(&mut srv, email, email, false);
    // 2. Validate link
    let mut response = srv.block_on(req.send()).unwrap();
    let _result: CommandResult = response.json().wait().expect("Could not parse json"); 
    // 3. Finish registration with user data
    let mut req: awc::ClientRequest = srv.post("/register/validate").timeout(Duration::new(15, 0));
    req = keep_session(response, req);
    let form = super::ValidateForm { username:  String::from("username"), password: String::from("passwd") };
    let mut response = srv.block_on(req.send_form(&form)).unwrap();
    assert!(response.status().is_success());
    let result: CommandResult = response.json().wait().expect("Could not parse json"); 
    assert!(!result.success);
    assert_eq!(Some(String::from("Username already taken")), result.error);

    // ================ Bad link
    //
    let emailbad = "emailo@test.fr";
    let req = register_link_mock(&mut srv, email, emailbad, false);
    let mut response2 = srv.block_on(req.send()).unwrap();
    assert!(response2.status().is_success());
    let result: CommandResult = response2.json().wait().expect("Could not parse json"); 
    assert!(!result.success);
    assert_eq!(Some(String::from("Incorrect link")), result.error);

    // ================ Link validity expired
    //
    let req = register_link_mock(&mut srv, email, email, true);
    let mut response = srv.block_on(req.send()).unwrap();
    // println!("response : {:#?}", response);
    assert!(response.status().is_success());
    let result: CommandResult = response.json().wait().expect("Could not parse json"); 
    assert!(!result.success);
    assert_eq!(Some(String::from("Link validity expired")), result.error);
}

fn keep_session(response: awc::ClientResponse<impl futures::stream::Stream>, request: awc::ClientRequest) -> awc::ClientRequest {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"actix-session=([^;]*)"#).unwrap();
    }
    let cookies = response.headers().get("set-cookie").unwrap().to_str().unwrap();
    let cookie_session : &str = RE.captures(cookies).unwrap().get(1).unwrap().as_str();
    request.cookie(
        awc::http::Cookie::build("actix-session", format!("{}", cookie_session))
        .path("/").secure(false).finish(),
        )
}

fn register_link_mock(srv: &mut actix_http_test::TestServerRuntime, email: &str, email_check: &str, expired: bool) -> awc::ClientRequest {
    let mut expires_at = super::Local::now().naive_local() + super::Duration::hours(24);
    if expired {
        expires_at = super::Local::now().naive_local() - super::Duration::hours(24);
    }
    let validate_params = format!("{}{}", email, expires_at.timestamp());
    let link = super::make_confirmation_data(&validate_params);
    let confirmation_hash = super::hash_password(&link)
        .map(|hash| super::to_url(&hash))
        .expect("Error hashing link");
    return srv.get(format!("/register/{}/{}/{}", confirmation_hash, email_check, expires_at.timestamp()))
        .timeout(Duration::new(15, 0));
}
