use chrono::Utc;
use uuid::Uuid;

use crate::{domain::account::Account, adapters::postgres};


pub async fn get_account(account_id: i32) -> Account{
       
    entity = postgres::create_account(entity).await.unwrap();

    Account::from_entity(entity)
}