pub mod schema;

use crate::{
    adapters::postgres::postgres_connection,
    adapters::rest::{rest_configuration},
    };
use actix_web::{App, HttpServer};
use super::util;

pub async fn start() -> std::io::Result<()>{
    
    util::load_env();

    let host = util::get_env_value("HOST");
    let port: u16 = util::get_env_value_u16("PORT");

    let db_url = util::get_env_value("DATABASE_URL");
    
    let pool = postgres_connection::get_pool();
    
    println!("Server running on port {}, database name {}", port, db_url);

    HttpServer::new(move ||{
                            App::new()
                            .app_data(pool.clone())
                            .configure(rest_configuration)
                        } 
                    )
        .bind((host, port))?
        .run()
        .await
}