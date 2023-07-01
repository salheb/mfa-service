pub mod health;

use actix_web::web;

pub fn rest_configuration(config: &mut web::ServiceConfig){
    // Add specific service routes bellow to map them into your rest API
    // Rest routes was separated in files but this is optional
    // TODO: add necessary API headers and pre-requisites, like CORS allowance params
    config.service(
        web::scope("/V1")
            .service(health::health)
    );
}