use actix::{Addr,SyncArbiter,Actor,SyncContext};
use diesel::prelude::{PgConnection, SqliteConnection};
use diesel::r2d2;
use dotenv;
use num_cpus;

#[cfg(not(any(feature = "sqlite", feature = "postgres")))]
compile_error!("Either feature \"sqlite\" or \"postgres\" must be enabled for this crate.");
#[cfg(all(feature = "sqlite", feature = "postgres"))]
compile_error!("Either feature \"sqlite\" or \"postgres\" must be enabled for this crate.");

#[cfg(all(feature = "sqlite", not(feature = "postgres")))]
pub type Connection = SqliteConnection;

#[cfg(all(not(feature = "sqlite"), feature = "postgres"))]
pub type Connection = PgConnection;

pub struct DbExecutor(pub r2d2::Pool<r2d2::ConnectionManager<Connection>>);

impl Actor for  DbExecutor{
    type Context = SyncContext<Self>;
}

#[cfg_attr(tarpaulin, skip)]
pub fn db_init() -> Addr<DbExecutor> {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // println!("Database : {}", db_url);
    let manager = r2d2::ConnectionManager::<Connection>::new(db_url);
    let pool = r2d2::Pool::builder().max_size(5).build(manager).expect("Failed to create pool.");
    SyncArbiter::start( num_cpus::get() * 4, move || { DbExecutor(pool.clone()) })
}

