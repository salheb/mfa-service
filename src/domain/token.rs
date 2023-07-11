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
    length: i32,
    #[serde(default)]
    created_at: NaiveDateTime,
    #[serde(default)]
    code: String
}

impl Token{
    pub fn from_entity(entity: TokenEntity) -> Token{
        Token { id: entity.id, uuid: entity.uuid, account: entity.account, 
                challenge_type: entity.challenge_type, sub_account: entity.sub_account, 
                phone_number: entity.phone_number, mail_address: entity.mail_address, 
                ttl: entity.ttl, length: entity.length, created_at: entity.created_at, 
                code: entity.code
            }
    }

    pub fn to_entity(model: &mut Token) -> TokenEntity{
        TokenEntity { id: model.id, uuid: model.uuid, account: model.account, 
                        challenge_type: model.challenge_type, sub_account: model.sub_account, 
                        phone_number: model.phone_number.clone(), mail_address: model.mail_address.clone(), 
                        ttl: model.ttl, length: model.length, 
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

    pub fn length(&mut self) -> &mut i32{
        &mut self.length
    }

    pub fn code(&mut self) -> &mut String{
        &mut self.code
    }

    pub fn challenge_type(&mut self) -> &mut i32{
        &mut self.challenge_type
    }

    pub fn response_fmt(&mut self) -> &mut Token{
        self.code = "".to_string();

        return self;
    }

}

