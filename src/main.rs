extern crate dotenv;

// project modules
mod adapters;
mod core;
mod domain;
mod usecases;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

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

    HttpServer::new(|| App::new().service(health))
        .bind((host, port))?
        .run()
        .await
}
