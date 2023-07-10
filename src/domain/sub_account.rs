use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::adapters::postgres::entity::sub_account_entity::SubAccountEntity;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SubAccount{
    #[serde(default)]
    id: i32,
    #[serde(default)]
    uuid: Uuid,
    name: String,
    mail_address: String,
    account_id: i32,
    #[serde(default)]
    created_at: NaiveDateTime
}

impl SubAccount{
    pub fn from_entity(sub_account_entity: SubAccountEntity) -> SubAccount{
        SubAccount {id: sub_account_entity.id, uuid: sub_account_entity.uuid, 
                    name: sub_account_entity.name, mail_address: sub_account_entity.mail_address,
                    account_id: sub_account_entity.account_id, created_at: sub_account_entity.created_at}
    }

    pub fn to_entity(model: &mut SubAccount) -> SubAccountEntity{
        SubAccountEntity {
            id: model.id,
            uuid: model.uuid,
            name: model.name.clone(),
            mail_address: model.mail_address.clone(),
            account_id: model.account_id,
            created_at: model.created_at
        }
    }

    pub fn created_at(&mut self) -> &mut NaiveDateTime{
        &mut self.created_at
    }

    pub fn uuid(&mut self) -> &mut Uuid{
        &mut self.uuid
    }

    pub fn account_id(&mut self) -> &mut i32{
        &mut self.account_id
    }
}