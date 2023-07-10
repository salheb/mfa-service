use actix_web::{error, post, web, HttpResponse, Error};
use futures::StreamExt;
use crate::usecases;

use crate::domain::sub_account::SubAccount;

const MAX_SIZE: usize = 262_144; // max payload size is 256k

// Create account endpoint
#[utoipa::path(
    path = "/sub-account",
    request_body = SubAccount,
    responses(
        (status = 200, description = "Created an account successfully", body = SubAccount),
        (status = 400, description = "Bad request - failed trying to parse payload", body = String)
    ),
    tag="sub_account"
)]
#[post("/sub-account")]
pub async fn create_sub_account(mut payload: web::Payload) -> Result<HttpResponse, Error>{
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
    let mut sub_account: SubAccount = match serde_json::from_slice::<SubAccount>(&body) {
       Ok(content) => content,
       Err(error) => return Err(error::ErrorBadRequest(error))
    };

    let result = usecases::create_sub_account::create_sub_account(&mut sub_account).await;
    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(res)), // <- send response    
        Err(e) => Err(error::ErrorNotFound(e.to_string()))
    }
    
}