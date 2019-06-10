use actix_web::web;
use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use uuid::Uuid;

use crate::wiring::DbPool;
use crate::errors::ServiceError;

use crate::schema::users::dsl;
use crate::users::models::{SlimUser, User, NewUser};

pub fn register_user(pool: web::Data<DbPool>, email: String, login: String, password: String) -> Result<SlimUser, ServiceError> {
    let conn = &pool.get().unwrap();
    let user = NewUser::with_details(login, email, password);
    #[cfg(not(test))]
    let inserted_user: User = diesel::insert_into(dsl::users).values(&user).get_result(conn)?;
    #[cfg(test)]
    diesel::insert_into(dsl::users).values(&user).execute(conn)?;
    #[cfg(test)]
    let inserted_user: User = dsl::users.order(dsl::id.desc()).first(conn)?;

    // let expire_date = (&inserted_user).expires_at.unwrap();
    return Ok(inserted_user.into());
}

pub fn validate_user(pool: web::Data<DbPool>, login: String) -> Result<(), ServiceError> {
    let conn = pool.get().unwrap();
    #[cfg(test)]
    diesel::update(dsl::users.filter(dsl::login.eq(login)))
        .set(dsl::active.eq(true))
        .execute(&conn)?;

    #[cfg(not(test))]
    let updated_row: Result<User, diesel::result::Error> = diesel::update(dsl::users.filter(dsl::login.eq(login)))
        .set(dsl::active.eq(true))
        .get_result(&conn);

    return Ok(());
}
