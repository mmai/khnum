use chrono::{Local, Duration, NaiveDateTime};
use std::convert::From;
use uuid::Uuid;

use crate::schema::{users};

#[derive(Debug, Serialize, Deserialize, Queryable)]
// XXX keep same field order as in schema.rs
pub struct User {
    pub id: i32,
    pub email: String,
    pub login: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub active: bool,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub login: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub active: bool,
    pub expires_at: Option<NaiveDateTime>,
}

impl NewUser {
    pub fn with_details(login: String, email: String, password: String) -> Self {
        NewUser {
            login,
            email,
            password,
            created_at: Local::now().naive_local(),
            active: false,
            expires_at: Some(Local::now().naive_local() + Duration::hours(24)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub login: String,
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser { 
            login: user.login,
            email: user.email,
        }
    }
}
