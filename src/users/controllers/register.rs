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

use crate::wiring::DbExecutor;

use crate::users::repository::register_handler::{RegisterUser, ValidateUser};
use crate::users::models::{SlimUser, User};
use crate::users::utils::hash_password;

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
    db: web::Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let user_data = user_data.into_inner();
    let hashed = hash_password(&user_data.password).expect("Error hashing password");
    let register_user = RegisterUser {
        email: user_data.email,
        login: user_data.login,
        password: hashed,
    };
    db.send(register_user)
        .from_err()
        .and_then(|db_response| match db_response {
            Ok((user, expires_at)) => {
                let res = send_confirmation(&user, &expires_at);
                Ok(HttpResponse::Ok().json(res))
            }
            Err(err) => {
                println!("Error when registering user : {}", err);
                Err(err.into())
            }
        })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendmailResult {
    result: bool
}

fn make_confirmation_link(login: &str) -> String {
    let key = dotenv::var("SECRET_KEY").unwrap();
    format!("{}{}", login, key)
}

fn send_confirmation(user: &SlimUser, expires_at: &NaiveDateTime) -> SendmailResult {
    let sending_email = std::env::var("SENDING_EMAIL_ADDRESS")
        .expect("SENDING_EMAIL_ADDRESS must be set");
    let base_url = dotenv::var("BASE_URL").unwrap_or_else(|_| "localhost".to_string());
    let recipient = user.email.as_str();
    let link = make_confirmation_link(&user.login);
    let hashlink = hash_password(&link).expect("Error hashing link");
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
    return SendmailResult {result: true};

    let result = mailer.send(email.unwrap().into());
    match result {
        Ok(_res) => SendmailResult {result: true} ,
        Err(error) => {
            println!("error \n {:#?}", error);
            SendmailResult {result: false}
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ValidateResult {
    message: String,
    result: bool
}

// ---------------- Validate action ------------
pub fn validate( data: web::Path<(String, String)>, db: web::Data<Addr<DbExecutor>>,) 
    // -> impl Future<Item = HttpResponse, Error = Error> {
    -> Box<Future<Item = HttpResponse, Error = Error>> {
    let hashlink = &data.0;
    let login = data.1.clone();
    let local_link = make_confirmation_link(&login);
    let is_valid = verify(local_link, hashlink).expect("Error using bcrypt");
    if !is_valid {
        return Box::new(result(Ok(HttpResponse::Ok().json(ValidateResult { result: false, message: String::from("Incorrect link") }))));
    }
    Box::new(db.send(ValidateUser { login: login })
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(_user) => { Ok(HttpResponse::Ok().json(ValidateResult { result: true, message: String::from("User activated") })) }
            Err(_err) => { Ok(HttpResponse::Ok().json(ValidateResult { result: false, message: String::from("User not found") })) }
        }))
}

#[cfg(test)]
mod tests {
    use crate::users;
    use actix_web::{web, test, http, App};
    use actix_http::HttpService;
    use actix_http_test::TestServer;
    use dotenv::dotenv;
    use std::time::Duration;
    use futures::future::Future;
    use super::SendmailResult;

    #[test]
    fn test() {
        dotenv().ok();
        let mut srv = TestServer::new( || {
            let conn_addr = crate::wiring::test_conn_init();
            HttpService::new(
                App::new().data(conn_addr.clone()).service(
                    web::resource("/register").route(
                        web::post().to_async(users::controllers::register::register)
                    )
                )
            )
        });

        let user = super::UserData {
            email: String::from("email@toto.fr"),
            login: String::from("login"),
            password: String::from("pass")
        };

        let req = srv.post("/register")
            .timeout(Duration::new(15, 0))
            .header( http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"),);

        let mut response = srv.block_on(req.send_json(&user)).unwrap();
        assert!(response.status().is_success());

        let res: SendmailResult = response.json().wait().expect("Could not parse");
        assert!(res.result);
    }

}
