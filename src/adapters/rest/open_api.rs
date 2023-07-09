use crate::{
    domain::{self},
    core::util,
    adapters::rest::account::__path_create_account,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
      paths(
        // Account
        create_account
      ),
      components(
            schemas(
                // Shared
                domain::account::Account
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