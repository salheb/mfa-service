use actix_web::{error, post, web, HttpResponse, Error};
use futures::StreamExt;
use crate::usecases;

use crate::domain::account::Account;

const MAX_SIZE: usize = 262_144; // max payload size is 256k

// Create account endpoint
#[utoipa::path(
    path = "/account",
    request_body = Account,
    responses(
        (status = 200, description = "Created an account successfully", body = Account)
    ),
    tag="account"
)]
#[post("/account")]
pub async fn create_account(mut payload: web::Payload) -> Result<HttpResponse, Error>{
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(content) = payload.next().await {
        let content = content?;
        // limit max size of in-memory payload
        if (body.len() + content.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&content);
    }

    // body is loaded, now we can deserialize serde-json
    let mut account: Account = serde_json::from_slice::<Account>(&body)?;
    account = usecases::create_account::create_account(account).await;
    Ok(HttpResponse::Ok().json(account)) // <- send response
}