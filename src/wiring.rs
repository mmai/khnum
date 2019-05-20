use actix::{Addr,SyncArbiter,Actor,SyncContext};
use diesel::prelude::PgConnection;
use diesel::r2d2;
use dotenv;
use num_cpus;

pub struct DbExecutor(pub r2d2::Pool<r2d2::ConnectionManager<PgConnection>>);

impl Actor for  DbExecutor{
    type Context = SyncContext<Self>;
}

pub fn db_init() -> Addr<DbExecutor> {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // println!("Database : {}", db_url);
    let manager = r2d2::ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder().max_size(5).build(manager).expect("Failed to create pool.");
    SyncArbiter::start( num_cpus::get() * 4, move || { DbExecutor(pool.clone()) })
}

