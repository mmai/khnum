use actix::Addr;
use actix_web::{ test, web, Error, HttpResponse, ResponseError, http};
use bcrypt::verify;
use chrono::{Duration, Local, NaiveDateTime};
use futures::future::{Future, result};

use url::form_urlencoded;

use lettre_email::Email;
use lettre::{SmtpClient, Transport};
use lettre::file::FileTransport;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::sendmail::SendmailTransport;

use crate::wiring::DbPool;

use crate::users::repository::{register_handler, fetch_handler};
use crate::users::models::{SlimUser, User};
use crate::users::utils::{hash_for_url, from_url};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    success: bool,
    error: Option<String>
}

// ---------------- Register Action------------

// UserData is used to extract data from a post request by the client
#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub email: String,
    pub login: String,
    pub password: String,
}

pub fn register(
    user_data: web::Json<UserData>,
    pool: web::Data<DbPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let user_data = user_data.into_inner();
    let res = check_existence(pool.clone(), &user_data.email, &user_data.login);
    match res {
        Ok(cde_res) => {
            if !cde_res.success {
                result(Ok(HttpResponse::Ok().json(cde_res)))
            } else {
                let hashed = hash_for_url(&user_data.password).expect("Error hashing password");
                // web::block( || register_handler::register_user(pool, user_data.email, user_data.login, hashed))
                    // .then(|res| { match res {
                let res = register_handler::register_user(pool, user_data.email, user_data.login, hashed);
                result(match res {
                        Ok((user, expires_at)) => {
                            let res = send_confirmation(&user, &expires_at);
                            Ok(HttpResponse::Ok().json(res))
                        }
                        Err(err) => {
                            println!("Error when registering user : {}", err);
                            Err(err.into())
                        }
                    })
                    // })
            }
        }
        Err(err) => {
           result(Err(err.into()))
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

fn make_confirmation_link(login: &str) -> String {
    let key = dotenv::var("SECRET_KEY").unwrap();
    format!("{}{}", login, key)
}

fn send_confirmation(user: &SlimUser, expires_at: &NaiveDateTime) -> CommandResult {
    let sending_email = std::env::var("SENDING_EMAIL_ADDRESS")
        .expect("SENDING_EMAIL_ADDRESS must be set");
    let base_url = dotenv::var("BASE_URL").unwrap_or_else(|_| "localhost".to_string());
    let recipient = user.email.as_str();
    let link = make_confirmation_link(&user.login);
    let hashlink = hash_for_url(&link).expect("Error hashing link");
    let url = format!("{}/register/{}/{}", base_url, hashlink, user.login);
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
struct ValidateResult {
    message: String,
    result: bool
}

// ---------------- Validate action ------------
pub fn validate( data: web::Path<(String, String)>, db: web::Data<DbPool>,) 
    // -> impl Future<Item = HttpResponse, Error = Error> {
    -> Box<Future<Item = HttpResponse, Error = Error>> {

    // Verify expiration date
    // TODO
    
    //Verify link
    let login = data.1.clone();
    let hashlink = from_url(&data.0);
    let local_link = make_confirmation_link(&login);
    let validate_result = verify(local_link, &hashlink[..])
        .map_err(|_err|
            CommandResult { success: false, error: Some(String::from("Invalid hash link")) }
        )
        .map(|is_valid| {
            if !is_valid {
                return CommandResult { success: false, error: Some(String::from("Incorrect link")) };
            }
            match register_handler::validate_user(db, login) {
                Ok(_user) => CommandResult { success: true, error: None },
                Err(_err) => CommandResult { success: false, error: Some(String::from("User not found")) }
            }
        });
    // println!("{:#?}", validate_result);
    match validate_result {
        Err(res) => Box::new(result(Ok(HttpResponse::Ok().json(res)))),
        Ok(res) => Box::new(result(Ok(HttpResponse::Ok().json(res))))
    }
}

#[cfg(test)]
mod tests {
    use crate::users;
    use actix_web::{web, test, http, App};
    use actix_http::HttpService;
    use actix_http_test::TestServer;
    use chrono::{NaiveDate, NaiveDateTime};
    use dotenv::dotenv;
    use std::time::Duration;
    use futures::future::Future;
    use super::CommandResult;

    use diesel::prelude::*;
    use crate::schema::users::dsl;
    use crate::users::models::{SlimUser, User, NewUser};

    #[test]
    fn test_register() {
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
                    web::resource("/register").route(
                        web::post().to_async(users::controllers::register::register)
                    )
                )
            )
        });

        //==== Test register
        let user = super::UserData {
            email: String::from("email2@toto.fr"),
            login: String::from("login2"),
            password: String::from("pass2")
        };

        let req = srv.post("/register")
            .timeout(Duration::new(15, 0))
            .header( http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"),);

        let mut response = srv.block_on(req.send_json(&user)).unwrap();
        assert!(response.status().is_success());
        let result: CommandResult = response.json().wait().expect("Could not parse json"); 
        assert!(result.success);

        //======== Test register with already registered email
        let existing_user = super::UserData {
            email: String::from("email@toto.fr"),
            login: String::from("login"),
            password: String::from("pass")
        };
        let req = srv.post("/register")
            .timeout(Duration::new(15, 0))
            .header( http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"),);

        let mut response = srv.block_on(req.send_json(&existing_user)).unwrap();
        assert!(response.status().is_success());
        let result: CommandResult = response.json().wait().expect("Could not parse json"); 
        assert!(!result.success);
        assert_eq!(Some(String::from("Email already taken")), result.error);
        
        //======== Test register with already registered login
        let existing_user = super::UserData {
            email: String::from("email3@toto.fr"),
            login: String::from("login"),
            password: String::from("pass")
        };
        let req = srv.post("/register")
            .timeout(Duration::new(15, 0))
            .header( http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"),);

        let mut response = srv.block_on(req.send_json(&existing_user)).unwrap();
        assert!(response.status().is_success());
        let result: CommandResult = response.json().wait().expect("Could not parse json"); 
        assert!(!result.success);
        assert_eq!(Some(String::from("Username already taken")), result.error);
    }

    #[test]
    fn test_validate() {
        dotenv().ok();
        let mut srv = TestServer::new( || {
            let pool = crate::wiring::test_conn_init();
            //Insert test data 
            let conn = &pool.get().unwrap();
            let user = NewUser {
                login: String::from("login"),
                email: String::from("email"),
                password: String::from("passwd"),
                created_at: NaiveDate::from_ymd(2019, 10, 10).and_hms(0,0,0),
                active: false,
                expires_at: Some(NaiveDate::from_ymd(2019, 10, 11).and_hms(0,0,0)),
            };
            diesel::insert_into(dsl::users).values(&user)
                .execute(conn).expect("Error populating test database");

            HttpService::new(
                App::new().data(pool.clone()).service(

             web::resource("/register/{hashlink}/{login}").route(
                 web::get().to_async(users::controllers::register::validate)
                    )
                )
            )
        });

        // Good link
        let login = "login";
        let link = super::make_confirmation_link(login);
        let hashlink = super::hash_for_url(&link).expect("Error hashing link");
        println!("hashlink : {}", hashlink);
        let req = srv.get(format!("/register/{}/{}", hashlink, login))
            .timeout(Duration::new(15, 0))
            .header( http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"),);

        let mut response = srv.block_on(req.send()).unwrap();
        // println!("response : {:#?}", response);
        assert!(response.status().is_success());
        let result: CommandResult = response.json().wait().expect("Could not parse json"); 
        assert!(result.success);

        // Bad link
        let login = "looogin";
        let req = srv.get(format!("/register/{}/{}", hashlink, login))
            .timeout(Duration::new(15, 0))
            .header( http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"),);

        let mut response2 = srv.block_on(req.send()).unwrap();
        assert!(response2.status().is_success());
        let result: CommandResult = response2.json().wait().expect("Could not parse json"); 
        assert!(!result.success);
        assert_eq!(Some(String::from("Incorrect link")), result.error);
    }

}
