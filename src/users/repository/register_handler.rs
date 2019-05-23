use actix::{Handler, Message};
use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use uuid::Uuid;

use crate::wiring::DbExecutor;
use crate::errors::ServiceError;

use crate::users::models::{SlimUser, User};
use crate::users::utils::hash_password;

// to be used to send data via the Actix actor system
#[derive(Debug)]
pub struct RegisterUser {
    pub email: String,
    pub login: String,
    pub password: String,
}

impl Message for RegisterUser {
    type Result = Result<SlimUser, ServiceError>;
}

impl Handler<RegisterUser> for DbExecutor {
    type Result = Result<SlimUser, ServiceError>;
    fn handle(&mut self, msg: RegisterUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::users;
        let conn = &self.0.get().unwrap();

        let user = User::with_details(msg.login, msg.email, msg.password);
        let inserted_user: User =
            diesel::insert_into(users).values(&user).get_result(conn)?;

        return Ok(inserted_user.into());
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
        use crate::schema::users::dsl::users;
        let conn = &self.0.get().unwrap();

        let updated_row = diesel::update(users.filter(login.eq(msg.login)))
            .set(active.eq(true))
            .get_result(conn);

        return Ok(());
    }
}
