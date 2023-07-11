use diesel::result::Error;
use totp_rs::{Rfc6238, TOTP};
use chrono::Utc;
use uuid::Uuid;

use crate::{domain::token::Token, adapters::postgres};

pub async fn generate_otp(otp: &mut Token)  -> Result<Token, Error>{

    let mut rfc = Rfc6238::with_defaults(
        "totp-sercret-123".as_bytes().to_vec()
    ).unwrap();
    
    // optional, set digits, issuer, account_name
    rfc.digits(otp.length().unsigned_abs().try_into().unwrap()).unwrap();
    
    let totp = TOTP::from_rfc6238(rfc).unwrap();
    println!("Generated Rfc6238 token: {} - {}",totp, totp.generate_current().unwrap());

    *otp.code() = totp.generate_current().unwrap();
    // TODO: implement oauth2 and get account parameter automatically to avoid sharing information between different clients
    *otp.account() = 1;
    *otp.created_at() = Utc::now().naive_utc();
    *otp.uuid() = Uuid::new_v4();

    let mut entity = Token::to_entity(otp);

    let new_entity = postgres::save_generated_totp(entity).await;

    match new_entity {
        Ok(ent) => entity = ent,
        Err(e) => return Err(e)
    };

    Ok(Token::from_entity(entity))
}
