use actix_web::{error, post, web, HttpResponse, Error};
use futures::StreamExt;
use crate::adapters::postgres::entity::token_entity::ChallengeType;
use crate::usecases;

use crate::domain::token::Token;

const MAX_SIZE: usize = 262_144; // max payload size is 256k

// Create account endpoint
#[utoipa::path(
    path = "/otp",
    request_body = Token,
    responses(
        (status = 200, description = "OTP generated successfully", body = Token),
        (status = 400, description = "Bad request - failed trying to parse payload", body = String)
    ),
    tag="otp"
)]
#[post("/otp")]
pub async fn otp_generate(mut payload: web::Payload) -> Result<HttpResponse, Error>{
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
    let mut otp: Token = match serde_json::from_slice::<Token>(&body) {
       Ok(content) => content,
       Err(error) => return Err(error::ErrorBadRequest(error))
    };

    match otp.length() {
        6...8 => (),
        _ => return Err(error::ErrorBadRequest("Invalid digits/length param. Supports only 6 to 8 digitis according to RFC6238."))        
    };
    let new_otp = usecases::generate_otp::generate_otp(&mut otp).await;
    match new_otp {
        Ok(mut res) => { 
            match ChallengeType::from_u32(res.challenge_type().clone().unsigned_abs()) {
                ChallengeType::API => Ok(HttpResponse::Ok().json(res)), // <- send response    
                ChallengeType::SMS => Ok(HttpResponse::Ok().json(res.response_fmt())), // <- send response    
                ChallengeType::MAIL => Ok(HttpResponse::Ok().json(res.response_fmt())), // <- send response    
                ChallengeType::WHATSAPP => Ok(HttpResponse::Ok().json(res.response_fmt())), // <- send response    
            }
        },
        Err(e) => Err(error::ErrorNotFound(e.to_string()))
    }
}