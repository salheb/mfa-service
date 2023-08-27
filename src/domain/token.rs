use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::adapters::postgres::entity::token_entity::TokenEntity;


#[derive(Serialize, Deserialize, ToSchema)]
pub struct Token{
    #[serde(default)]
    id: i64,
    #[serde(default)]
    uuid: Uuid,
    #[serde(default)]
    account: i32,
    challenge_type: i32,
    sub_account: i32,
    #[serde(default)]
    phone_number: String,
    #[serde(default)]
    mail_address: String,
    ttl: i32,
    digits: i32,
    #[serde(default)]
    created_at: NaiveDateTime,
    #[serde(default)]
    code: String,
    #[serde(default)]
    text_template: String
}

impl Token{
    pub fn from_entity(entity: TokenEntity) -> Token{
        Token { id: entity.id, uuid: entity.uuid, account: entity.account, 
                challenge_type: entity.challenge_type, sub_account: entity.sub_account, 
                phone_number: entity.phone_number, mail_address: entity.mail_address, 
                ttl: entity.ttl, digits: entity.digits, created_at: entity.created_at, 
                code: entity.code, text_template: "".to_string()
            }
    }

    pub fn to_entity(model: &mut Token) -> TokenEntity{
        TokenEntity { id: model.id, uuid: model.uuid, account: model.account, 
                        challenge_type: model.challenge_type, sub_account: model.sub_account, 
                        phone_number: model.phone_number.clone(), mail_address: model.mail_address.clone(), 
                        ttl: model.ttl, digits: model.digits, 
                        created_at: model.created_at, code: model.code.clone()}
    }

    pub fn created_at(&mut self) -> &mut NaiveDateTime{
        &mut self.created_at
    }

    pub fn uuid(&mut self) -> &mut Uuid{
        &mut self.uuid
    }

    pub fn account(&mut self) -> &mut i32{
        &mut self.account
    }

    pub fn sub_account(&mut self) -> &mut i32{
        &mut self.sub_account
    }

    pub fn digits(&mut self) -> &mut i32{
        &mut self.digits
    }

    pub fn code(&mut self) -> &mut String{
        &mut self.code
    }

    pub fn empty_code(&mut self){
        self.code.clear();
    }

    pub fn challenge_type(&mut self) -> &mut i32{
        &mut self.challenge_type
    }

    pub fn text_template(&mut self) -> &mut String{
        &mut self.text_template
    }

}


#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct TokenCandidate{
    #[serde(default)]
    uuid: Uuid,
    account: i32,
    sub_account: i32,
    #[serde(default)]
    ttl: i32,
    #[serde(default)]
    code: String,
    #[serde(default)]
    offline: bool,
    #[serde(default)]
    pub validated: bool,
    #[serde(default)]
    pub digits: i32
}

impl TokenCandidate{

    pub fn offline(&self) -> bool{
        self.offline
    }

    pub fn sub_account(&self) -> i32{
        self.sub_account
    }

    pub fn code(&self) -> String{
        self.code.clone()
    }

    pub fn uuid(&self) -> Uuid{
        self.uuid
    }

    pub fn digits(&self) -> i32{
        self.digits
    }
}