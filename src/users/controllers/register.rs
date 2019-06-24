use actix_session::{CookieSession, Session};
use actix_web::{ test, web, Error, error, HttpResponse, ResponseError, http};
use bcrypt::verify;
use chrono::{Duration, Local, NaiveDateTime};
use futures::future::{Future, result, err};

use url::form_urlencoded;

use lettre_email::Email;
use lettre::{SmtpClient, Transport};
use lettre::file::FileTransport;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::sendmail::SendmailTransport;

use crate::wiring::DbPool;
use crate::errors::ServiceError;

use crate::users::repository::{register_handler, fetch_handler};
use crate::users::models::{SlimUser, User};
use crate::users::utils::{hash_password, to_url, from_url};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    success: bool,
    error: Option<String>
}

// ---------------- Request Action------------

// UserData is used to extract data from a post request by the client
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestForm {
    email: String
}

pub fn request(
    form_data: web::Form<RequestForm>,
    pool: web::Data<DbPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let form_data = form_data.into_inner();
    let res = check_email_available(pool.clone(), &form_data.email);
    match res {
        Ok(cde_res) => {
            if !cde_res.success {
                result(Ok(HttpResponse::Ok().json(cde_res)))
            } else {
                let expires_at = Local::now().naive_local() + Duration::hours(24);
                let res = send_confirmation(form_data.email, expires_at);
                result(Ok(HttpResponse::Ok().json(res)))
            }
        }
        Err(err) => {
           result(Err(err.into()))
        }
    }
}

// ---------------- Validate link action ------------
pub fn validate_link( 
    session: Session,
    data: web::Path<(String, String, String)>, 
    db: web::Data<DbPool>,) 
    // -> impl Future<Item = HttpResponse, Error = Error> {
    -> Box<Future<Item = HttpResponse, Error = Error>> {

    //Verify link
    let hashlink = from_url(&data.0);
    let email = from_url(&data.1);
    let expires_at: i64 = data.2.clone().parse().unwrap();
    let validate_params = format!("{}{}", email, expires_at);
    let local_link = make_confirmation_data(&validate_params);
    let validate_result = verify(local_link, &hashlink[..])
        .map_err(|_err|
            CommandResult { success: false, error: Some(String::from("Invalid hash link")) }
        )
        .map(|is_valid| {
            if !is_valid {
                return CommandResult { success: false, error: Some(String::from("Incorrect link")) };
            }
            let now = Local::now().naive_local().timestamp();
            if expires_at < now {
                return CommandResult { success: false, error: Some(String::from("Link validity expired")) };
            }
            if session.set("email", email ).is_ok() {
                // println!("email in session : {:#?}", session.get::<String>("email").expect("err while getting email from session"));
                CommandResult { success: true, error: None }
            } else {
                CommandResult { success: false, error: Some(String::from("Could not save email in session")) }
            }
        });
    // println!("{:#?}", validate_result);
    match validate_result {
        Err(res) => Box::new(result(Ok(HttpResponse::Ok().json(res)))),
        Ok(res) => Box::new(result(Ok(HttpResponse::Ok().json(res))))
    }
}

// -------- 
pub fn register(
    session: Session,
    form_data: web::Form<ValidateForm>,
    pool: web::Data<DbPool>,
// ) -> impl Future<Item = HttpResponse, Error = ServiceError> {
) -> impl Future<Item = HttpResponse, Error = Error> {
    let form_data = form_data.into_inner();

    let res = {
        let opt = session.get::<String>("email").expect("could not get session email");
        let email = opt.unwrap();
        let cde_res = check_existence(pool.clone(), &email, &form_data.username).expect("error when checking existence");
        if !cde_res.success {
            Ok(HttpResponse::Ok().json(cde_res))
        } else {
            let user = register_handler::register_user(pool, email, form_data.username, form_data.password).expect("error when inserting new user");
            Ok( HttpResponse::Ok().json(CommandResult {success: true, error: None}))
        }
    };

    // let res = match session.get::<String>("email") {
    //     Err(err) => {println!("error when session.get"); Err(err.into())},
    //     Ok(opt) => match opt {
    //         None => {println!("email not found in  session"); Err(error::CookieParseError::MissingPair.into())},
    //         // None => result(Err(Error::InternalServerError)),
    //         Some(email) => {
    //             check_existence(pool.clone(), &email, &form_data.username)
    //                 .map_err(|err| { println!("error when checking existence"); err.into()  })
    //                 .map(|cde_res| {
    //                     if !cde_res.success {
    //                         HttpResponse::Ok().json(cde_res)
    //                     } else {
    //                         let expires_at = Local::now().naive_local() + Duration::hours(24);
    //
    //                         let res = register_handler::register_user(pool, email, form_data.username, form_data.password);
    //                         match res {
    //                             Ok(user) => {
    //                                 HttpResponse::Ok().json(res)
    //                             }
    //                             Err(err) => {
    //                                 println!("Error when registering user : {}", err);
    //                                 Err(err.into())
    //                             }
    //                         }
    //                     }
    //                 } )
    //         }
    //     }
    // };
    result(res)
}



fn check_email_available(pool: web::Data<DbPool>, email: &String) -> Result<CommandResult, Error> {
    let res = fetch_handler::email_exists(pool, email);
    match res {
        Ok(email_exists) => {
            if email_exists {
                return Ok(CommandResult {success: false, error: Some(String::from("Email already taken"))});
            }
            Ok(CommandResult {success: true, error: None})
        }
        Err(err) => {
            println!("Error when looking unicity : {}", err);
            Err(err.into())
        }
    }
}

fn check_existence(pool: web::Data<DbPool>, email: &String, login: &String) -> Result<CommandResult, Error> {
    let res = fetch_handler::fetch(pool, email, login);
    match res {
        Ok(users) => {
            if users.len() == 0 {
                return Ok(CommandResult {success: true, error: None});
            }
            let mut err_message = "Username already taken";
            let same_email: Vec<&SlimUser> = users.iter().filter(|user| &user.email == email).collect();
            if same_email.len() > 0 {
                err_message = "Email already taken";
            }
            Ok(CommandResult {success: false, error: Some(String::from(err_message))})
        }
        Err(err) => {
            println!("Error when looking unicity : {}", err);
            Err(err.into())
        }
    }
}

fn make_confirmation_data(msg: &str) -> String {
    let key = dotenv::var("SECRET_KEY").unwrap();
    format!("{}{}", msg, key)
}

fn send_confirmation(email: String, expires_at: NaiveDateTime) -> CommandResult {
    let validate_params = format!("{}{}", email, expires_at.timestamp());
    // println!("{}{}", email, expires_at.timestamp());

    let sending_email = std::env::var("SENDING_EMAIL_ADDRESS")
        .expect("SENDING_EMAIL_ADDRESS must be set");
    let base_url = dotenv::var("BASE_URL").unwrap_or_else(|_| "localhost".to_string());
    let recipient = &email[..];
    let link = make_confirmation_data(&validate_params);
    let confirmation_hash = hash_password(&link)
        .map(|hash| to_url(&hash))
        .expect("Error hashing link");
    let url = format!("{}/register/{}/{}/{}", base_url, confirmation_hash, to_url(&email), expires_at.timestamp());
    let email_body = format!(
        "Please click on the link below to complete registration. <br/>
         <a href=\"{url}\">{url}</a> <br>
         your Invitation expires on <strong>{date}</strong>",
         url = url,
         date = expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string()
    );
    // println!("{}", email_body);
    // println!("{}", recipient);

    let email = Email::builder()
        .from((sending_email, "Activue"))
        .to(recipient)
        .subject("You have been invited to join Activue")
        .html(email_body)
        .build();
    assert!(email.is_ok());

    // let smtp_login = dotenv::var("SMTP_LOGIN").unwrap_or_else(|_| "user".to_string());
    // let smtp_pass = dotenv::var("SMTP_PASSWORD").unwrap_or_else(|_| "password".to_string());
    // let smtp_server = dotenv::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.localhost".to_string()); 
    // let creds = Credentials::new( smtp_login, smtp_pass );
    // let mut mailer = SmtpClient::new_simple(&smtp_server)
    //     .unwrap()
    //     .credentials(creds)
    //     .transport();

    // let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();
    let sendmail = dotenv::var("SENDMAIL").unwrap_or_else(|_| "/usr/sbin/sendmail".to_string()); 
    let mut mailer = SendmailTransport::new_with_command(sendmail);

    // We don't send the mail in test environment
    #[cfg(test)]
    return CommandResult {success: true, error: None};

    let result = mailer.send(email.unwrap().into());
    match result {
        Ok(_res) => CommandResult {success: true, error: None} ,
        Err(error) => {
            // println!("error \n {:#?}", error);
            CommandResult {success: false, error: Some(format!("Error sending mail. {:#?}", error))}
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateForm {
    username: String,
    password: String,
}

#[cfg(test)]
mod tests {
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

    // fn get_session_cookie<'req>(response: awc::ClientResponse) -> actix_http::cookie::Cookie<'req> {
    //     lazy_static! {
    //         static ref RE: Regex = Regex::new(r#"actix_session=([^;]*)"#).unwrap();
    //     }
    //     let cookies = response.headers().get("set-cookie").unwrap().to_str().unwrap();
    //     let cookie_session:&str = RE.captures(cookies).unwrap()
    //         .get(1).unwrap()
    //         .as_str();
    //     awc::http::Cookie::build("actix-session", format!("{}", cookie_session)).path("/").secure(false).finish()
    // }
    
    // fn keep_session<'req>(response: awc::ClientResponse<impl futures::stream::Stream>, request: &'req mut awc::ClientRequest){
    //     lazy_static! {
    //         static ref RE: Regex = Regex::new(r#"actix_session=([^;]*)"#).unwrap();
    //     }
    //     let cookies = response.headers().get("set-cookie").unwrap().to_str().unwrap();
    //     let cookie_session:&str = RE.captures(cookies).unwrap()
    //         .get(1).unwrap()
    //         .as_str();
    //     request.cookie(
    //             awc::http::Cookie::build("actix-session", format!("{}", cookie_session))
    //             .path("/").secure(false).finish(),
    //         );
    // }

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
        let expires_at = super::Local::now().naive_local() + super::Duration::hours(24);
        let validate_params = format!("{}{}", email, expires_at.timestamp());
        let link = super::make_confirmation_data(&validate_params);
        let confirmation_hash = super::hash_password(&link)
            .map(|hash| super::to_url(&hash))
            .expect("Error hashing link");
        // 2. Validate link
        let req = srv.get(format!("/register/{}/{}/{}", confirmation_hash, email, expires_at.timestamp()))
            .timeout(Duration::new(15, 0));
        let mut response = srv.block_on(req.send()).unwrap();
        assert!(response.status().is_success());
        // println!("response : {:#?}", response);
        let result: CommandResult = response.json().wait().expect("Could not parse json"); 
        // println!("err: {}", result.error.unwrap_or(String::from("none")));
        assert!(result.success);

        // 3. Finish registration with user data
        let mut req: awc::ClientRequest = srv.post("/register/validate").timeout(Duration::new(15, 0));
        // keep_session(response, &mut req);
        // req = req.cookie(get_session_cookie(response));
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"actix-session=([^;]*)"#).unwrap();
        }
        let cookies = response.headers().get("set-cookie").unwrap().to_str().unwrap();
        let cookie_session = RE.captures(cookies).unwrap().get(1).unwrap().as_str();
        req = req.cookie(
            awc::http::Cookie::build("actix-session", format!("{}", cookie_session))
            .path("/").secure(false).finish(),
            );

        let form = super::ValidateForm { username:  String::from("username"), password: String::from("passwd") };
        let mut response = srv.block_on(req.send_form(&form)).unwrap();
        assert!(response.status().is_success());
        let result: CommandResult = response.json().wait().expect("Could not parse json"); 
        assert!(result.success);

        //Registering with same email should now fail
        let formRequest = super::RequestForm { email:  String::from(email) };
        let reqRequest = srv.post("/register/request").timeout(Duration::new(15, 0));
        let mut response = srv.block_on(reqRequest.send_form(&formRequest)).unwrap();
        // println!("response : {:#?}", response);
        assert!(response.status().is_success());
        let result: CommandResult = response.json().wait().expect("Could not parse json"); 
        assert!(!result.success);
        assert_eq!(Some(String::from("Email already taken")), result.error);

        // ================ Bad link
        //
        let emailbad = "emailo@test.fr";
        let req = srv.get(format!("/register/{}/{}/{}", confirmation_hash, emailbad, expires_at.timestamp()))
            .timeout(Duration::new(15, 0));
        let mut response2 = srv.block_on(req.send()).unwrap();
        assert!(response2.status().is_success());
        let result: CommandResult = response2.json().wait().expect("Could not parse json"); 
        assert!(!result.success);
        assert_eq!(Some(String::from("Incorrect link")), result.error);

        // ================ Link validity expired
        //
        let expires_at = super::Local::now().naive_local() - super::Duration::hours(24);
        let validate_params = format!("{}{}", email, expires_at.timestamp());
        let link = super::make_confirmation_data(&validate_params);
        let confirmation_hash = super::hash_password(&link)
            .map(|hash| super::to_url(&hash))
            .expect("Error hashing link");
        let req = srv.get(format!("/register/{}/{}/{}", confirmation_hash, email, expires_at.timestamp()))
            .timeout(Duration::new(15, 0));
        let mut response = srv.block_on(req.send()).unwrap();
        // println!("response : {:#?}", response);
        assert!(response.status().is_success());
        let result: CommandResult = response.json().wait().expect("Could not parse json"); 
        assert!(!result.success);
        assert_eq!(Some(String::from("Link validity expired")), result.error);
    }

}
