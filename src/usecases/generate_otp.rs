use log::{info, warn};
use totp_rs::{Rfc6238, TOTP, Secret};
use chrono::Utc;
use uuid::Uuid;

use crate::{domain::{token::Token, sub_account::SubAccount, message::Message, errors::{TokenNotGeneratedError, SubAccountNotFoundError, TokenNotPersistedError}}, adapters::{postgres::{self, entity::token_entity::ChallengeType}, kafka}, core::util::{is_dev_environment, get_env_value}};

pub async fn generate_otp(otp: &mut Token)  -> Result<Token, String>{

    let sub_account_entity = postgres::get_sub_account(*otp.sub_account()).await;
    let mut sub_account: SubAccount;
    match sub_account_entity {
        Ok(res) => { sub_account = SubAccount::from_entity(res) },
        Err(_) => { return Err(SubAccountNotFoundError.to_string()) }
    }
    
    let encoded_secret = Secret::Raw(sub_account.otp_secret().as_bytes().to_vec());
    let mut rfc = Rfc6238::with_defaults(encoded_secret.to_bytes().unwrap()).unwrap();    
    rfc.digits(otp.digits().unsigned_abs().try_into().unwrap_or(6)).unwrap();
    
    
    let totp = TOTP::from_rfc6238(rfc).unwrap();    

    *otp.code() = totp.generate_current().unwrap();

    // check if template text is present, otherwise use default
    match otp.text_template().is_empty(){
        true => get_env_value("OTP_MESSAGE"),
        false => otp.text_template().clone()
    };

    if is_dev_environment()
    {
        info!("Generated Rfc6238 token: {} - {}",totp, *otp.code());
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
        }
        Err(_) => {
            warn!("Failed to save token");
            return Err(TokenNotPersistedError.to_string())
        }
    }

    match ChallengeType::from_u32((*model.challenge_type()).unsigned_abs()) {
        ChallengeType::Api => 
        {
            info!("Challenge_type = API");
            Ok(model)
        }, 
        _ => 
        {
            info!("Challenge_type <> API");
            model.empty_code();
            match send_message(&mut model).await {
                true =>{
                    Ok(model)
                },
                false =>{
                    Err(TokenNotGeneratedError.to_string())
                }
            }            
        }
    }
}

async fn send_message(token: &mut Token) -> bool{
    //TODO: implement messaging (kafka, rabbit) implementation to send messages asyncronously
    info!("Starting generate_otp::send_message fn");
    let message_content = get_text(token.text_template().to_string(), token.code().to_string());

    let new_message = Message::new(*token.account(), 
                                            *token.challenge_type(), 
                                            token.phone_number().to_string(), 
                                            token.mail_address().to_string(), 
                                            message_content.to_string());
    
    match kafka::message(new_message).await {
        true => 
                {
                    info!("Message sent: {} | with code {}",message_content, &mut token.code());
                    true
                },
        false => 
                {
                    warn!("Message not sent");
                    false
                },
    }
}

fn get_text(template: String, code: String) -> String{
    template.replace("{}", &code)
}