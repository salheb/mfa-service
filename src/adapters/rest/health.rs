use actix_web::{get, Responder, HttpResponse};

// Simple health check API
#[get("/health")]
pub async fn health() -> impl Responder{
    HttpResponse::Ok().json("I'm alive")
}