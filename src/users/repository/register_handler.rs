use actix::{Handler, Message};
use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use uuid::Uuid;

use crate::wiring::DbExecutor;
use crate::errors::ServiceError;

use crate::schema::users::dsl::*;
use crate::users::models::{SlimUser, User, NewUser};

// to be used to send data via the Actix actor system
#[derive(Debug)]
pub struct RegisterUser {
    pub email: String,
    pub login: String,
    pub password: String,
}

impl Message for RegisterUser {
    type Result = Result<(SlimUser, NaiveDateTime), ServiceError>;
}

impl Handler<RegisterUser> for DbExecutor {
    type Result = Result<(SlimUser, NaiveDateTime), ServiceError>;
    fn handle(&mut self, msg: RegisterUser, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get().unwrap();

        let user = NewUser::with_details(msg.login, msg.email, msg.password);
        let inserted_user: User =
            diesel::insert_into(users).values(&user).get_result(conn)?;
        let expire_date = (&inserted_user).expires_at.unwrap();

        return Ok((inserted_user.into(), expire_date));
    }
}


#[derive(Debug)]
pub struct ValidateUser {
    pub login: String,
}

impl Message for ValidateUser {
    type Result = Result<(), ServiceError>;
}

impl Handler<ValidateUser> for DbExecutor {
    type Result = Result<(), ServiceError>;
    fn handle(&mut self, msg: ValidateUser, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get().unwrap();

        let updated_row: Result<User, diesel::result::Error> = diesel::update(users.filter(login.eq(msg.login)))
            .set(active.eq(true))
            .get_result(conn);

        return Ok(());
    }
}
