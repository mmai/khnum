use chrono::{Local, Duration, NaiveDateTime};
use std::convert::From;
use uuid::Uuid;

use crate::schema::{users};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub login: String,
    pub email: String,
    pub password: String,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

impl User {
    pub fn with_details(login: String, email: String, password: String) -> Self {
        User {
            login,
            email,
            password,
            active: false,
            created_at: Local::now().naive_local(),
            expires_at: Local::now().naive_local() + Duration::hours(24),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub login: String,
    pub email: String,
    pub expires_at: NaiveDateTime,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser { 
            login: user.login,
            email: user.email,
            expires_at: user.expires_at,
        }
    }
}
