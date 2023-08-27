pub mod health;
pub mod account;
pub mod sub_account;
pub mod open_api;
pub mod token;

use actix_web::web;

use self::open_api::with_swagger;

pub fn rest_configuration(config: &mut web::ServiceConfig){
    // Add specific service routes bellow to map them into your rest API
    // Rest routes was separated in files but this is optional
    // TODO: add necessary API headers and pre-requisites, like CORS allowance params
    // TODO: Add actix-web-middleware-keycloak-auth to enforce OAuth2 within keycloak
    config.service(
        web::scope("/v1")
            .service(health::health)
            .service(account::create_account)
            .service(sub_account::create_sub_account)
            .service(token::otp_generate)
            .service(token::otp_validate)
            .service(with_swagger())
    );
}