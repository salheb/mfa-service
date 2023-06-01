use diesel::{pg::PgConnection, r2d2::ConnectionManager};

type DatabasePool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct PostgresConnection{
    pub database_url: String,
    pub database_name: String
}

impl PostgresConnection{
    pub fn get_pool(&self) -> DatabasePool{
        let database = format!("{}/{}", &self.database_url, &self.database_name);

        let manager = ConnectionManager::<PgConnection>::new(&database);
        
        r2d2::Pool::new(manager).unwrap()
    }
}