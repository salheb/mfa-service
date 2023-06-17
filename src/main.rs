extern crate dotenv;

// project modules
mod adapters;
mod core;
mod domain;
mod usecases;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use crate::adapters::postgres::postgres_connection::PostgresConnection;

// Simple health check API
// TODO: move to rest adapter module
#[get("/health")]
async fn health() -> impl Responder{
    HttpResponse::Ok().json("I'm alive")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load env file based on runtime arguments
    core::util::load_env();

    //let host = env::var("HOST").expect("HOST must be set.");
    let host = core::util::get_env_value("HOST");
    let port: u16 = core::util::get_env_value_u16("PORT");

    let db_name = core::util::get_env_value("DATABASE_NAME");
    let db_url = core::util::get_env_value("DATABASE_URL");

    //let db = PostgresConnection{database_url: db_url.to_string(), database_name: db_name.to_string()};
    
    //println!("Server running on port {}, database name {}", port, db_name);

    HttpServer::new(|| App::new().service(health))
        .bind((host, port))?
        .run()
        .await
}