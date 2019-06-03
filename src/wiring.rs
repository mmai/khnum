use actix::{Addr,SyncArbiter,Actor,SyncContext};
use actix::{Handler, Message, prelude::SendError};
use diesel::prelude::{Connection, PgConnection, SqliteConnection};
use diesel::r2d2;
use num_cpus;
use futures::future::Future;
use crate::errors::ServiceError;

// #[cfg(not(any(feature = "sqlite", feature = "postgres")))]
// compile_error!("Either feature \"sqlite\" or \"postgres\" must be enabled for this crate.");
// #[cfg(all(feature = "sqlite", feature = "postgres"))]
// compile_error!("Features \"sqlite\" and \"postgres\" should not be enabled at the same time.");

// #[cfg(all(feature = "sqlite", not(feature = "postgres")))]
#[cfg(test)]
pub type MyConnection = SqliteConnection;

// #[cfg(all(not(feature = "sqlite"), feature = "postgres"))]
#[cfg(not(test))]
pub type MyConnection = PgConnection;

pub struct DbExecutor(pub r2d2::Pool<r2d2::ConnectionManager<MyConnection>>);

impl Actor for  DbExecutor{
    type Context = SyncContext<Self>;
}

#[cfg_attr(tarpaulin, skip)]
pub fn db_init(db_url: String) -> Addr<DbExecutor> {
    // println!("Database : {}", db_url);
    let manager = r2d2::ConnectionManager::<MyConnection>::new(db_url);
    let pool = r2d2::Pool::builder().max_size(5).build(manager).expect("Failed to create pool.");
    SyncArbiter::start( num_cpus::get() * 4, move || { DbExecutor(pool.clone()) })
}

// ================== Test database initialization
// #[cfg(feature = "sqlite")]
#[cfg(test)]
embed_migrations!("migrations/sqlite");

// migration actix message
#[cfg(test)]
#[derive(Debug)]
pub enum DoMigrations { DoMigrations }

#[cfg(test)]
impl Message for DoMigrations {
    type Result = Result<(), ServiceError>;
}

#[cfg(test)]
impl Handler<DoMigrations> for DbExecutor {
    type Result = Result<(), ServiceError>;
    fn handle(&mut self, msg: DoMigrations, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get().unwrap();
        embedded_migrations::run(conn);
        return Ok(());
    }
}

// #[cfg(feature = "sqlite")]
#[cfg(test)]
pub fn test_conn_init() -> Addr<DbExecutor> {
    let manager = r2d2::ConnectionManager::<MyConnection>::new(":memory:");
    let pool = r2d2::Pool::builder().max_size(1).build(manager).expect("Failed to create pool.");
    let db_pool = SyncArbiter::start( 1, move || { DbExecutor(pool.clone()) });
    let _ = db_pool.do_send(DoMigrations::DoMigrations);
    db_pool
}

