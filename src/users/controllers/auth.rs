use actix_web::middleware::identity::Identity;
use actix_session::{CookieSession, Session};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder, ResponseError};
use futures::future::{Future, result};

use crate::wiring::DbPool;
use crate::errors::ServiceError;

use crate::users::repository::auth_handler;
use crate::users::utils::create_token;
use crate::users::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
    login: String,
    password: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct CommandResult {
//     success: bool,
//     error: Option<String>
// }

pub fn login(
    auth_data: web::Form<AuthData>,
    session: Session,
    id: Identity,
    db: web::Data<DbPool>,
    ) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    let data: AuthData = auth_data.into_inner();

    web::block( move || auth_handler::auth(db, data.login, data.password))
        .then(move |res| { 
            match res {
            Ok(user) => {
                //Via jwt
                let token = create_token(&user)?;
                id.remember(token);
                //Via session cookie
                if session.set("user", user).is_ok() {
                    // ServiceError::InternalServerError
                }
                Ok(HttpResponse::Ok().into())
            }
            Err(err) => {
                panic!("{:?}", err);
                Ok(err.error_response())
            }
        }})
}

pub fn logout(
    session: Session,
    id: Identity) -> impl Responder {
    session.clear();
    id.forget();
    HttpResponse::Ok()
}

pub fn get_me(
    session: Session,
    // logged_user: auth_handler::LoggedUser
    ) -> HttpResponse {
    // ) -> impl Future<Item = HttpResponse, Error = Error> {
        let opt = session.get::<models::SlimUser>("user").expect("could not get session user");
        match opt {
            // Ok(user) => Ok(HttpResponse::Ok().json(user)),
            // Err(err) => Ok(err.error_response())
            Some(user) => HttpResponse::Ok().json(user),
            None => HttpResponse::Unauthorized().json("Unauthorized")
        }
        // let user = opt.unwrap();
    // HttpResponse::Ok().json(logged_user)
}

#[cfg(test)]
mod tests;
