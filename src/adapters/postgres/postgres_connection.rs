use diesel::{pg::PgConnection, r2d2::ConnectionManager};

type DatabasePool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct PostgresConnection{
    pub database_url: String
}

impl PostgresConnection{
    pub fn get_pool(&self) -> DatabasePool{
        let manager = ConnectionManager::<PgConnection>::new(&self.database_url);
        
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failure building Postgres connection pool.")
    }
}