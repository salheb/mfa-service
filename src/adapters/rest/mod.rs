pub mod health;

use actix_web::web;

pub fn rest_configuration(config: &mut web::ServiceConfig){
    config.service(
        web::scope("/V1")
            .service(health::health)
    );
}