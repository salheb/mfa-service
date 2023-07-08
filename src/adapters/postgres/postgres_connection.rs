use diesel::{pg::PgConnection, r2d2::ConnectionManager};

use crate::core::util;


pub fn get_pool() -> r2d2::Pool<ConnectionManager<PgConnection>>{
    let database_url = util::get_env_value("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    r2d2::Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failure building Postgres connection pool.")
}
