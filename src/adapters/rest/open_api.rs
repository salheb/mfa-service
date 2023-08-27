use crate::{
    domain::{self},
    core::util,
    adapters::rest::account::__path_create_account,
    adapters::rest::sub_account::__path_create_sub_account,
    adapters::rest::token::__path_otp_generate,
    adapters::rest::token::__path_otp_validate,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
      paths(
        create_account,
        create_sub_account,
        otp_generate,
        otp_validate
      ),
      components(
            schemas(
                // Shared
                domain::account::Account,
                domain::sub_account::SubAccount,
                domain::token::Token,
                domain::token::TokenCandidate
            )
      ),
      tags(
          (name = "OTP", description = "OTP API endpoints")
      ),
)]
struct ApiDoc;

/// Configure SwaggerUI using `utoipa`
pub fn with_swagger() -> SwaggerUi {
    let host = util::get_env_value("HOST");
    let port: u16 = util::get_env_value_u16("PORT");

    let openapi = ApiDoc::openapi();


    println!("Visit Swagger UI at http://{}:{}/swagger-ui/#", host, port);

    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone())
}