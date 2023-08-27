use diesel::result::Error;
use totp_rs::{Rfc6238, TOTP, Secret};
use chrono::Utc;
use uuid::Uuid;


use crate::{domain::{token::Token, sub_account::SubAccount}, adapters::postgres::{self, entity::token_entity::ChallengeType}, core::util::{is_dev_environment, get_env_value}};

pub async fn generate_otp(otp: &mut Token)  -> Result<Token, Error>{

    let sub_account_entity = postgres::get_sub_account(*otp.sub_account()).await;
    let mut sub_account: SubAccount;
    match sub_account_entity {
        Ok(res) => { sub_account = SubAccount::from_entity(res) },
        Err(err) => { return Err(err) }
    }
    
    let encoded_secret = Secret::Raw(sub_account.otp_secret().as_bytes().to_vec());
    let mut rfc = Rfc6238::with_defaults(encoded_secret.to_bytes().unwrap()).unwrap();    
    rfc.digits(otp.digits().unsigned_abs().try_into().unwrap_or(6)).unwrap();
    
    
    let totp = TOTP::from_rfc6238(rfc).unwrap();    

    *otp.code() = totp.generate_current().unwrap();

    // check if template text is present, otherwise use default
    let message_template = match otp.text_template().is_empty(){
        true => get_env_value("OTP_MESSAGE"),
        false => otp.text_template().clone()
    };

    if is_dev_environment()
    {
        println!("Generated Rfc6238 token: {} - {}",totp, *otp.code());
    }

    // TODO: implement oauth2 and get account parameter automatically to avoid sharing information between different customers
    *otp.account() = 1;
    *otp.created_at() = Utc::now().naive_utc();
    *otp.uuid() = Uuid::new_v4();

    let entity = Token::to_entity(otp);

    let new_entity = postgres::save_generated_totp(entity).await;
    let mut model: Token;
    match new_entity {
        Ok(ent) => {
            model = Token::from_entity(ent);
            match ChallengeType::from_u32((*model.challenge_type()).unsigned_abs()) {
                ChallengeType::Api => {}, 
                _ => { 
                        send_message(ChallengeType::from_u32(
                                                    (*model.challenge_type()).unsigned_abs()), 
                                                    message_template, 
                                                    model.code().to_string()
                                                ).await;
                        model.empty_code();
                     }
            };
        },
        Err(e) => return Err(e)
    };    

    Ok(model)
}

async fn send_message(challenge: ChallengeType, message: String, code: String){
    //TODO: implement messaging (kafka, rabbit) implementation to send messages asyncronously

    match challenge {
        ChallengeType::Api => {}, 
        ChallengeType::Sms => {}, 
        ChallengeType::Mail => {}, 
        ChallengeType::Whatsapp => {},    
    }

    if is_dev_environment()
    {
        println!("Message sent: {} | with code {}",message, code);
    }
}
