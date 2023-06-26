// project modules
mod adapters;
mod core;
mod domain;
mod usecases;

use actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    // start api server
    core::app::start().await
}