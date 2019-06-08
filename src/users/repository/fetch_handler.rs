use actix::{Handler, Message};
use actix_web::{web, dev::Payload, Error, HttpRequest};
use actix_web::{middleware::identity::Identity, FromRequest};
use bcrypt::verify;
use diesel::prelude::*;

use crate::wiring::DbPool;

use crate::errors::ServiceError;
use crate::users::models::{SlimUser, User};
use crate::users::utils::decode_token;
use crate::wiring::MyConnection;

pub fn fetch(pool: web::Data<DbPool>, email: &String, login: &String) -> Result<Vec<SlimUser>, ServiceError> {
    use crate::schema::users::dsl;
    let conn: &MyConnection = &pool.get().unwrap();
    let items = dsl::users.filter(
        dsl::active.eq(true)
        .and(
            dsl::email.eq(email)
            .or( dsl::login.eq(login))
        )
    ).load::<User>(conn)?;
    return Ok(items.into_iter().map(|item| item.into()).collect());
}
