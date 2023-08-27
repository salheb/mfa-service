use actix_web::{error, post, web, HttpResponse, Error};
use futures::StreamExt;
use crate::usecases::validate_otp::validate_token;
use crate::usecases;

use crate::domain::token::{Token, TokenCandidate};

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[utoipa::path(
    post,
    path = "/otp/generate",
    request_body = Token,
    responses(
        (status = 200, description = "OTP generated successfully", body = Token),
        (status = 400, description = "Bad request - failed trying to parse payload", body = String)
    ),
    params(
        ("phone_number" = String, Path, description = "Phone number in full international format"),
        ("sub_account" = i32, Path, description = "Numeric sub account identification"),
        ("challenge_type" = i32, Path, description = "API = 1, SMS = 2, MAIL = 3, WHATSAPP = 4"),
        ("mail_address" = String, Path, description = "Mail address"),
        ("ttl" = i32, Path, description = "Time to live - a time in seconds which an online token can be validated."),
        ("digits" = i32, Path, description = "Generated code digits between 6 and 8"),
    )
)]
#[post("/otp/generate")]
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
       _ => return Err(error::ErrorBadRequest("Invalid request payload structure."))
    };

    

    match otp.digits() {
        6...8 => (),
        _ => return Err(error::ErrorBadRequest("Invalid digits/length param. Supports only 6 to 8 digitis according to RFC6238."))        
    };
    let new_otp = usecases::generate_otp::generate_otp(&mut otp).await;
    
    Ok(HttpResponse::Ok().json(new_otp.unwrap()))
    //Err(e) => Err(error::ErrorNotFound(e.to_string()))
}


#[utoipa::path(
    path = "/otp/validate",
    request_body = TokenCandidate,
    responses(
        (status = 200, description = "OTP validated successfully", body = String),
        (status = 400, description = "Bad request - failed trying to parse payload", body = String)
    ),
    params(
        //otp_uuid: uuid::Uuid,code: String, account: i32, sub_account: i32, offline: bool
        ("otp_uuid" = uuid::Uuid, Path, description = "Token uuid to be validated"),
        ("code" = String, Path, description = "Token uuid to be validated"),
        ("account" = i32, Path, description = "Account number to be validated"),
        ("sub_account" = i32, Path, description = "Sub Account number to be validated"),
        ("offline" = String, Path, description = "true = OFFLINE MODE, false = ONLINE MODE"),
    )
)]
#[post("/otp/validate")]
pub async fn otp_validate(mut payload: web::Payload) -> Result<HttpResponse, Error>{

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
    let mut otp: TokenCandidate = match serde_json::from_slice::<TokenCandidate>(&body) {
        Ok(body_content) => body_content,
        Err(result) => 
            {
                println!("Error while deserializing json : {}", result);
                return Err(error::ErrorBadRequest("Invalid request payload structure."))
            }
    };

    otp.validated = validate_token(&otp).await;

    Ok(HttpResponse::Ok().json(otp))
}