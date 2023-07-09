use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use crate::adapters::postgres::entity::account_entity::AccountEntity;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Account{
    #[serde(default)]
    id: Option<i32>,
    #[serde(default)]
    uuid: Uuid,
    name: String,
    mail_address: String,
    #[serde(default)]
    created_at: NaiveDateTime
}

impl Account{
    pub fn new(id: Option<i32>,
        uuid: Uuid,
        name: String,
        mail_address: String,
        created_at: NaiveDateTime) -> Account{
            Account {id, uuid, name, mail_address, created_at}
    }

    // basic mapping functions
    pub fn from_entity(account_entity: AccountEntity) -> Account{
        Account { 
            id: Some(account_entity.id),          
            uuid: account_entity.uuid, 
            name: account_entity.name, 
            mail_address: account_entity.mail_address, 
            created_at: account_entity.created_at 
            }
    }

    pub fn to_entity(model: Account) -> AccountEntity{
        AccountEntity { 
            id: match model.id{
                Some(id) => id,
                None => 0
            },  
            uuid: model.uuid, 
            name: model.name, 
            mail_address: model.mail_address, 
            created_at: model.created_at 
            }
    }

    pub fn created_at(&mut self) -> &mut NaiveDateTime{
        &mut self.created_at
    }

    pub fn uuid(&mut self) -> &mut Uuid{
        &mut self.uuid
    }

    pub fn id(&mut self) -> &mut Option<i32>{
        &mut self.id
    }
}