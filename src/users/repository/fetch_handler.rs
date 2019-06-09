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

pub fn email_exists(pool: web::Data<DbPool>, email: &String) -> Result<bool, ServiceError> {
    use crate::schema::users::dsl;
    let conn: &MyConnection = &pool.get().unwrap();
    // let items = dsl::users.filter( dsl::email.eq(email)).load::<User>(conn)?;
    // Ok(items.into_iter().map(|item| item.into()).collect())
    diesel::dsl::select(diesel::dsl::exists(dsl::users.filter(dsl::email.eq(email))))
        .get_result(conn)
        .map_err(|err| err.into())
    // match (res){
    //     Ok(exists) => Ok(exists),
    //     diesel::result::Error => ServiceError(err)
    // }
}

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
