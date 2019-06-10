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
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub login: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub active: bool,
}

impl NewUser {
    pub fn with_details(login: String, email: String, password: String) -> Self {
        NewUser {
            login,
            email,
            password,
            created_at: Local::now().naive_local(),
            active: false,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[test]
fn user_with_details() {
    let u = NewUser::with_details(String::from("login"), String::from("email@toto.fr"), String::from("pass"));

    assert_eq!(u.login , String::from("login"));
}

#[test]
fn slim_from_user() {
    let u = User {
        id: 1,
        login: "login".into(),
        email: "email@toto.fr".into(),
        password: "pass".into(),
        created_at: Local::now().naive_local(),
        active: false,
    };
    let s:SlimUser = u.into();
    assert_eq!(SlimUser { login: "login".into(), email: "email@toto.fr".into() }, s);
}
