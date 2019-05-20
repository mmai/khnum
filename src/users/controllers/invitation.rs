use actix::Addr;
use actix_web::{web, Error, HttpResponse, ResponseError};
use futures::future::Future;
use lettre_email::Email;
use lettre::{SmtpClient, Transport};
use lettre::file::FileTransport;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::sendmail::SendmailTransport;
use dotenv::dotenv;

use crate::wiring::DbExecutor;

use crate::users::repository::invitation_handler::CreateInvitation;
use crate::users::models::Invitation;

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    result: bool
}

pub fn register_email(
    signup_invitation: web::Json<CreateInvitation>,
    db: web::Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(signup_invitation.into_inner())
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(invitation) => {
                let res = send_invitation(&invitation);
                Ok(HttpResponse::Ok().json(res))
            }
            Err(err) => Ok(err.error_response()),
        })
}

pub fn send_invitation(invitation: &Invitation) -> Result {
    let sending_email = std::env::var("SENDING_EMAIL_ADDRESS")
        .expect("SENDING_EMAIL_ADDRESS must be set");
    let base_url = dotenv::var("BASE_URL").unwrap_or_else(|_| "localhost".to_string());
    let recipient = invitation.email.as_str();
    let email_body = format!(
        "Please click on the link below to complete registration. <br/>
         <a href=\"{url}/register.html?id={id}&email={email}\">
         {url}/register</a> <br>
         your Invitation expires on <strong>{date}</strong>",
         url = base_url,
         id = invitation.id,
         email = invitation.email,
         date = invitation
            .expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string()
    );
    // println!("{}", email_body);

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

    let result = mailer.send(email.unwrap().into());

    match result {
        Ok(res) => Result {result: true} ,
        Err(error) => {
            println!("error \n {:#?}", error);
            Result {result: false}
        }
    }
}
