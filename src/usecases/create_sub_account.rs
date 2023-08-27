use diesel::result::Error;

use chrono::Utc;
use uuid::Uuid;

use crate::{domain::sub_account::SubAccount, adapters::postgres};

use super::get_account;


pub async fn create_sub_account(sub_account: &mut SubAccount) -> Result<SubAccount, Error>{

    //first check if associated account exists
    let account = get_account::get_account(*sub_account.account_id()).await;

    let _account = match account {
        Ok(acc) => acc,
        Err(e) => {
            return Err(e)
        },
    };

    *sub_account.created_at() = Utc::now().naive_utc();
    *sub_account.uuid() = Uuid::new_v4();

    let mut entity = SubAccount::to_entity(sub_account);
    
    let new_entity = postgres::create_sub_account(entity).await;

    match new_entity {
        Ok(ent) => entity = ent,
        Err(e) => return Err(e)
    }


    Ok(SubAccount::from_entity(entity))
}