use chrono::Utc;
use totp_rs::{Rfc6238, Secret, TOTP};

use crate::{adapters::postgres, domain::{sub_account::SubAccount, token::TokenCandidate}, core::util::is_dev_environment};

pub async fn validate_token(token_candidate: &TokenCandidate) -> bool{
    
    //TODO: check if account belongs to oauth_token owner

    if token_candidate.offline() {
        let sub_account_entity = postgres::get_sub_account(token_candidate.sub_account()).await;
        let mut sub_account: SubAccount;
        match sub_account_entity {
            Ok(res) => { sub_account = SubAccount::from_entity(res) },
            Err(res) => {
                    if is_dev_environment()
                    {
                        println!("Failed to get subaccount : {}", res);
                    }
                    return false;
                }
        }
        
        let encoded_secret = Secret::Raw(sub_account.otp_secret().as_bytes().to_vec());
        let mut rfc = Rfc6238::with_defaults(encoded_secret.to_bytes().unwrap()).unwrap();
        
        if token_candidate.digits().eq(&0) {
            rfc.digits(6).unwrap();
        }
        else{
            rfc.digits(usize::try_from(token_candidate.digits()).unwrap_or(6)).unwrap();
        }
        
        
        
        let totp = TOTP::from_rfc6238(rfc).unwrap();    

        let actual_code = totp.generate_current().unwrap();

        if is_dev_environment()
        {
            println!("{}", totp);
            println!("TOTP generated: {}", &actual_code);
            println!("TOTP candidate: {}", &token_candidate.code());
        }

        actual_code.eq(&token_candidate.code())
    }
    else{
        let now = Utc::now().naive_utc();
        let otp = postgres::get_totp(token_candidate.uuid()).await;

        match otp {
            Ok(res) => { 
                (res.code.eq(&token_candidate.code())) && ((now - res.created_at).num_seconds() < res.ttl.into())
            },
            _ => {false}
        }
    }
}