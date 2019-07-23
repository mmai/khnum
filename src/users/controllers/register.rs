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

use crate::users::repository::user_handler;
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
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    // panic!("in request ");
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
           result(Err(err))
        }
    }
}

// ---------------- Validate link action ------------
pub fn validate_link( 
    session: Session,
    data: web::Path<(String, String, String)>, 
    ) 
    // -> impl Future<Item = HttpResponse, Error = Error> {
    -> Box<Future<Item = HttpResponse, Error = ServiceError>> {

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
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateForm {
    username: String,
    password: String,
}

pub fn register(
    session: Session,
    form_data: web::Form<ValidateForm>,
    pool: web::Data<DbPool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
// ) -> impl Future<Item = HttpResponse, Error = Error> {
    let form_data = form_data.into_inner();

    let res = {
        let opt = session.get::<String>("email").expect("could not get session email");
        let email = opt.unwrap();
        let cde_res = check_existence(pool.clone(), &email, &form_data.username).expect("error when checking existence");
        if !cde_res.success {
            Ok(HttpResponse::Ok().json(cde_res))
        } else {
            let _user = user_handler::add(pool, email, form_data.username, form_data.password).expect("error when inserting new user");
            Ok( HttpResponse::Ok().json(CommandResult {success: true, error: None}))
        }
    };
    result(res)
}

fn check_email_available(pool: web::Data<DbPool>, email: &String) -> Result<CommandResult, ServiceError> {
    let res = user_handler::email_exists(pool, email);
    match res {
        Ok(email_exists) => {
            if email_exists {
                return Ok(CommandResult {success: false, error: Some(String::from("Email already taken"))});
            }
            Ok(CommandResult {success: true, error: None})
        }
        Err(err) => {
            println!("Error when looking unicity : {}", err);
            Err(err)
        }
    }
}

fn check_existence(pool: web::Data<DbPool>, email: &String, login: &String) -> Result<CommandResult, ServiceError> {
    let res = user_handler::fetch(pool, email, login);
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
            Err(err)
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

    // We don't send the mail in test environment
    #[cfg(test)]
    return CommandResult {success: true, error: None};

    // let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();
    let sendmail = dotenv::var("SENDMAIL").unwrap_or_else(|_| "/usr/sbin/sendmail".to_string()); 
    let mut mailer = SendmailTransport::new_with_command(sendmail);

    let result = mailer.send(email.unwrap().into());
    match result {
        Ok(_res) => CommandResult {success: true, error: None} ,
        Err(error) => {
            // println!("error \n {:#?}", error);
            CommandResult {success: false, error: Some(format!("Error sending mail. {:#?}", error))}
        }
    }
}

#[cfg(test)]
mod tests;
// #[path = "./register_test.rs"] // avoid creating a /register folder
// mod register_test;
