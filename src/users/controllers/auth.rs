use actix_web::middleware::identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder, ResponseError};
use futures::future::{Future, result};

use crate::wiring::DbPool;

use crate::users::repository::auth_handler;
use crate::users::utils::create_token;

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
    id: Identity,
    db: web::Data<DbPool>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
    let data: AuthData = auth_data.into_inner();

    web::block( move || auth_handler::auth(db, data.login, data.password))
        .then(move |res| { 
            match res {
            Ok(user) => {
                let token = create_token(&user)?;
                id.remember(token);
                Ok(HttpResponse::Ok().into())
            }
            Err(err) => Ok(err.error_response())
        }})
}

pub fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok()
}

pub fn get_me(logged_user: auth_handler::LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}

#[cfg(test)]
mod tests;
